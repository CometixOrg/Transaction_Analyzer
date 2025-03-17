use {
    crate::{address::fetch_transaction_record_with_context, analyzer_program::id},
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
    cometix_sdk::SDK
};

const DEFAULT_ANALYSIS_CONTEXT_ID: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");

fn generate_transaction_payload(
    initiator_address: &Pubkey,
    subject_address: &Pubkey,
    asset_address: &Pubkey,
    protocol_context: &Pubkey,
    instruction_type: u8,
) -> Instruction {
    let derived_account = fetch_transaction_record_with_context(
        subject_address,
        asset_address,
        protocol_context,
    );

    assert!(instruction_type <= 1);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(*initiator_address, true),
            AccountMeta::new(derived_account, false),
            AccountMeta::new_readonly(*subject_address, false),
            AccountMeta::new_readonly(*asset_address, false),
            AccountMeta::new_readonly(DEFAULT_ANALYSIS_CONTEXT_ID, false),
            AccountMeta::new_readonly(*protocol_context, false),
        ],
        data: vec![instruction_type],
    }
}

pub fn submit_transaction_record(
    initiator_address: &Pubkey,
    subject_address: &Pubkey,
    asset_address: &Pubkey,
    protocol_context: &Pubkey,
) -> Instruction {
    generate_transaction_payload(
        initiator_address,
        subject_address,
        asset_address,
        protocol_context,
        0,
    )
}

pub fn submit_transaction_record_idempotent(
    initiator_address: &Pubkey,
    subject_address: &Pubkey,
    asset_address: &Pubkey,
    protocol_context: &Pubkey,
) -> Instruction {
    generate_transaction_payload(
        initiator_address,
        subject_address,
        asset_address,
        protocol_context,
        1,
    )
}

pub fn remap_transaction_path(
    subject_address: &Pubkey,
    context_asset: &Pubkey,
    nested_asset: &Pubkey,
    protocol_context: &Pubkey,
) -> Instruction {
    let base_trace = fetch_transaction_record_with_context(
        subject_address,
        context_asset,
        protocol_context,
    );
    let destination_trace = fetch_transaction_record_with_context(
        subject_address,
        nested_asset,
        protocol_context,
    );
    let nested_trace = fetch_transaction_record_with_context(
        &base_trace,
        nested_asset,
        protocol_context,
    );

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(nested_trace, false),
            AccountMeta::new_readonly(*nested_asset, false),
            AccountMeta::new(destination_trace, false),
            AccountMeta::new_readonly(base_trace, false),
            AccountMeta::new_readonly(*context_asset, false),
            AccountMeta::new(*subject_address, true),
            AccountMeta::new_readonly(*protocol_context, false),
        ],
        data: vec![2],
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        solana_program::system_program,
    };

    #[test]
    fn validate_default_context_id() {
        assert_eq!(system_program::id(), DEFAULT_ANALYSIS_CONTEXT_ID);
    }
}
