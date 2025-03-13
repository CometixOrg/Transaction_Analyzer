use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransactionEvent {
    pub event_type: String,
    pub transaction_id: Pubkey,
    pub timestamp: u64,
    pub details: String,
}

impl TransactionEvent {
    pub fn new(event_type: String, transaction_id: Pubkey, timestamp: u64, details: String) -> Self {
        TransactionEvent {
            event_type,
            transaction_id,
            timestamp,
            details,
        }
    }
}
