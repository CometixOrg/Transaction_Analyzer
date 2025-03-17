use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransactionAnalyzerInstruction {
    Submit,
    SubmitIdempotent,
    RedirectNested,
    LogEvent,
}

impl TransactionAnalyzerInstruction {
    pub fn log_event(event: &TransactionEvent) -> Self {
        TransactionAnalyzerInstruction::LogEvent
    }
}
