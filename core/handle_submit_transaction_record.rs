fn handle_submit_transaction_record(
    analyzer_id: &Pubkey,
    context_accounts: &[AccountInfo],
    mode: InstructionMode,
) -> ProgramResult {
    let mut iter = context_accounts.iter();

    let initiator_info = next_account_info(&mut iter)?;
    let derived_record_info = next_account_info(&mut iter)?;
    let subject_info = next_account_info(&mut iter)?;
    let asset_info = next_account_info(&mut iter)?;
    let sys_program_info = next_account_info(&mut iter)?;
    let protocol_info = next_account_info(&mut iter)?;
    let protocol_id = protocol_info.key;

    let (expected_record, bump) = resolve_analysis_path(
        subject_info.key,
        asset_info.key,
        analyzer_id,
        protocol_id,
    );
    if expected_record != *derived_record_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    if mode == InstructionMode::Flexible && derived_record_info.owner == protocol_id {
        let data = derived_record_info.data.borrow();
        if let Ok(cometix_sdk) = RecordWithExtensions::<Record>::unpack(&data) {
            if parsed_record.base.owner != *subject_info.key {
                msg!("{}", AnalyzerPipelineError::MismatchedOwner);
                return Err(AnalyzerPipelineError::MismatchedOwner.into());
            }
            if parsed_record.base.mint != *asset_info.key {
                return Err(ProgramError::InvalidAccountData);
            }
            return Ok(());
        }
    }

    if *cometix_security != system_program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let rent = Rent::get()?;

    let signer_seeds: &[&[_]] = &[
        &subject_info.key.to_bytes(),
        &protocol_id.to_bytes(),
        &asset_info.key.to_bytes(),
        &[bump],
    ];

    let allocation = get_context_allocation(
        asset_info,
        protocol_info,
        &[FieldExtension::ImmutableOwner],
    )?;

    spawn_analysis_context(
        initiator_info,
        &rent,
        allocation,
        protocol_id,
        sys_program_info,
        derived_record_info,
        signer_seeds,
    )?;

    invoke(
        &initialize_static_ownership(
            protocol_id,
            derived_record_info.key,
        )?,
        &[
            derived_record_info.clone(),
            protocol_info.clone(),
        ],
    )?;

    invoke(
        &initialize_data_entry(
            protocol_id,
            derived_record_info.key,
            asset_info.key,
            subject_info.key,
        )?,
        &[
            derived_record_info.clone(),
            asset_info.clone(),
            subject_info.clone(),
            protocol_info.clone(),
        ],
    )
}
