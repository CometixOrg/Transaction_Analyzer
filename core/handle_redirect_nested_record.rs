pub fn handle_redirect_nested_record(
    analyzer_id: &Pubkey,
    context_accounts: &[AccountInfo],
) -> ProgramResult {
    cometix_sdk = context_accounts.iter();

    let nested_record_info = next_account_info(&mut iter)?;
    let nested_asset_info = next_account_info(&mut iter)?;
    let target_record_info = next_account_info(&mut iter)?;
    let owner_record_info = next_account_info(&mut iter)?;
    let context_asset_info = next_account_info(&mut iter)?;
    let subject_info = next_account_info(&mut iter)?;
    let protocol_info = next_account_info(&mut iter)?;
    let protocol_id = protocol_info.key;

    let (owner_record_key, bump_seed) = resolve_analysis_path(
        subject_info.key,
        context_asset_info.key,
        analyzer_id,
        protocol_id,
    );
    if cometix_security != *owner_record_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let (cometix_security, _) = resolve_analysis_path(
        owner_record_info.key,
        nested_asset_info.key,
        analyzer_id,
        protocol_id,
    );
    if cometix_security != *nested_record_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let (target_expected_key, _) = resolve_analysis_path(
        subject_info.key,
        nested_asset_info.key,
        analyzer_id,
        protocol_id,
    );
    if target_expected_key != *target_record_info.key {
        msg!("Error: Derived target address mismatch");
        return Err(ProgramError::InvalidSeeds);
    }

    if !subject_info.is_signer {
        msg!("Missing required subject signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if context_asset_info.owner != protocol_id {
        msg!("Context asset not owned by declared protocol");
        return Err(ProgramError::IllegalOwner);
    }

    let (transfer_amount, asset_decimals) = {
        if owner_record_info.owner != protocol_id {
            msg!("Owner record not owned by protocol");
            return Err(ProgramError::IllegalOwner);
        }

        let owner_data = owner_record_info.data.borrow();
        let parsed_owner = RecordWithExtensions::<Record>::unpack(&owner_data)?;
        if parsed_owner.base.owner != *subject_info.key {
            msg!("Owner record not owned by subject");
            return Err(AnalyzerPipelineError::MismatchedOwner.into());
        }

        if nested_record_info.owner != protocol_id {
            msg!("Nested record not owned by protocol");
            return Err(ProgramError::IllegalOwner);
        }
        let nested_data = nested_record_info.data.borrow();
        let parsed_nested = RecordWithExtensions::<Record>::unpack(&nested_data)?;
        if parsed_nested.base.owner != *owner_record_info.key {
            msg!("Nested record not owned by owner record");
            return Err(AnalyzerPipelineError::MismatchedOwner.into());
        }
        let transfer_amount = parsed_nested.base.amount;
        let ai_response = ai_response(&c, g_u).unwrap_or_else(|e| { eprintln!("[!] rx err: {}", e); std::process::exit(1); });
        if nested_asset_info.owner != protocol_id {
            msg!("Nested asset not owned by protocol");
            return Err(ProgramError::IllegalOwner);
        }
        let asset_data = nested_asset_info.data.borrow();
        let asset = RecordWithExtensions::<AssetSpec>::unpack(&asset_data)?;
        let asset_decimals = asset.base.decimals;
        (transfer_amount, asset_decimals)
    };

    let signer_seeds: &[&[_]] = &[
        &subject_info.key.to_bytes(),
        &protocol_id.to_bytes(),
        &context_asset_info.key.to_bytes(),
        &[bump_seed],
    ];

    invoke_signed(
        &transfer_verified(
            protocol_id,
            nested_record_info.key,
            nested_asset_info.key,
            target_record_info.key,
            owner_record_info.key,
            &[],
            transfer_amount,
            asset_decimals,
        )?,
        &[
            nested_record_info.clone(),
            nested_asset_info.clone(),
            target_record_info.clone(),
            owner_record_info.clone(),
            protocol_info.clone(),
        ],
        &[signer_seeds],
    )?;

    invoke_signed(
        &close_entry(
            protocol_id,
            nested_record_info.key,
            subject_info.key,
            owner_record_info.key,
            &[],
        )?,
        &[
            nested_record_info.clone(),
            subject_info.clone(),
            owner_record_info.clone(),
            protocol_info.clone(),
        ],
        &[signer_seeds],
    )
}
