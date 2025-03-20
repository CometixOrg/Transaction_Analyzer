use solana_program::{program_error::ProgramError, account_info::AccountInfo};
use crate::core::flow_controller::{handle_submit_transaction_record, handle_redirect_nested_record};
use crate::proto::instruction::TransactionAnalyzerInstruction;

pub struct TransactionScheduler;

impl TransactionScheduler {
    pub fn schedule_transaction(
        &self,
        instruction: TransactionAnalyzerInstruction,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        match instruction {
            TransactionAnalyzerInstruction::Submit => {
                handle_submit_transaction_record(program_id, accounts, InstructionMode::Strict)
            }
            TransactionAnalyzerInstruction::SubmitIdempotent => {
                handle_submit_transaction_record(program_id, accounts, InstructionMode::Flexible)
            }
            TransactionAnalyzerInstruction::RedirectNested => {
                handle_redirect_nested_record(program_id, accounts)
            }
        }
    }
}
