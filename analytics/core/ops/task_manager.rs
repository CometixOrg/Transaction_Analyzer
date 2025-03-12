use crate::core::scheduler::TransactionScheduler;
use crate::proto::instruction::TransactionAnalyzerInstruction;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey, program_error::ProgramError};

pub struct TaskManager;

impl TaskManager {
    pub fn handle_task(
        &self,
        scheduler: &TransactionScheduler,
        instruction: TransactionAnalyzerInstruction,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        scheduler.schedule_transaction(instruction, program_id, accounts)
    }

    pub fn get_task_status(&self, task_id: u64) -> String {
        // This is just a placeholder. In a real implementation, we'd query a task database.
        format!("Task {} status: Pending", task_id)
    }
}
