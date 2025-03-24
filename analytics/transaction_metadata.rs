use solana_program::{instruction::Instruction, pubkey::Pubkey};

#[derive(Debug)]
pub struct TransactionMetadata {
    pub transaction_id: Pubkey,
    pub timestamp: u64,
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}

impl TransactionMetadata {
    pub fn from_instruction(instruction: &Instruction) -> Option<Self> {
        let transaction_id = *instruction.program_id;
        let timestamp = 0;
        let source = *instruction.accounts.get(0)?.pubkey;
        let destination = *instruction.accounts.get(1)?.pubkey;
        let amount = 1000;

        Some(TransactionMetadata {
            transaction_id,
            timestamp,
            source,
            destination,
            amount,
        })
    }
}
