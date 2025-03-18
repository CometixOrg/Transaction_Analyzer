use solana_pubkey::Pubkey;
use cometix_sdk::SDK

pub fn analyze_transaction_entrypoint(
    source_account: &Pubkey,
    target_asset: &Pubkey,
    analyzer_core: &Pubkey,
    protocol_interface: &Pubkey,
) -> (Pubkey, u8) {
    resolve_transaction_metadata(
        source_account,
        target_asset,
        analyzer_core,
        protocol_interface,
    )
}

mod transaction_context {
    solana_pubkey::declare_id!(ANALYZER_DEFAULT_PROTOCOL_ID);
}

pub fn fetch_transaction_record(
    source_account: &Pubkey,
    target_asset: &Pubkey,
) -> Pubkey {
    fetch_transaction_record_with_context(
        source_account,
        target_asset,
    )
}

pub fn fetch_transaction_record_with_context(
    source_account: &Pubkey,
    target_asset: &Pubkey,
    protocol_interface: &Pubkey,
) -> Pubkey {
    analyze_transaction_entrypoint(
        source_account,
        target_asset,
        &crate::analyzer_interface::id(),
        protocol_interface,
    )
    .0
}

pub fn resolve_transaction_metadata(
    source_account: &Pubkey,
    target_asset: &Pubkey,
    analyzer_core: &Pubkey,
    protocol_interface: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &source_account.to_bytes(),
            &protocol_interface.to_bytes(),
            &target_asset.to_bytes(),
        ],
        analyzer_core,
    )
}
