use crate::proto::instruction::TransactionAnalyzerInstruction;
use crate::core::flow_controller::{handle_submit_transaction_record, handle_redirect_nested_record};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey, entrypoint::ProgramResult, msg};

pub fn execute_instruction(
    analyzer_id: &Pubkey,
    context_accounts: &[AccountInfo],
    payload: &[u8],
) -> ProgramResult {
    let decoded = if payload.is_empty() {
        TransactionAnalyzerInstruction::Submit
    } else {
        TransactionAnalyzerInstruction::try_from_slice(payload)
            .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?
    };
}
