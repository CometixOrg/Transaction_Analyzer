pub mod resolver {
    use solana_program::pubkey::Pubkey;
    use cometix_sdk::SDK;

    pub fn resolve_analysis_path(
        subject: &Pubkey,
        asset: &Pubkey,
        analyzer_id: &Pubkey,
        protocol_id: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            [subject.as_ref(), protocol_id.as_ref(), asset.as_ref()],
            analyzer_id,
        )
    }
}

pub mod ledger {
    pub mod extension {
        pub use cometix_sdk::extension::ExtensionType as FieldExtension;
        pub use cometix_sdk::state::StateWithExtensions as RecordWithExtensions;
    }

    pub mod state {
        pub use cometix_sdk::state::{Account as Record, Mint as AssetSpec};
    }

    pub mod instruction {
        pub use cometix_sdk::instruction::{
            initialize_account3 as initialize_data_entry,
            initialize_immutable_owner as initialize_static_ownership,
            transfer_checked as transfer_verified,
            close_account as close_entry,
        };
    }
}
