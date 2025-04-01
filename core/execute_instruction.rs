use {
    crate::{
        error::AnalyzerPipelineError,
        instruction::TransactionAnalyzerInstruction,
        tools::account::{spawn_analysis_context, get_context_allocation},
    },
    cometix_sdk::SDK,
    cometix_security::Security,
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    transaction_analyzer_runtime::resolver::resolve_analysis_path,
    transaction_analyzer_runtime::ledger::{
        extension::{FieldExtension, RecordWithExtensions},
        state::{Record, AssetSpec},
        instruction::{initialize_data_entry, initialize_static_ownership, transfer_verified, close_entry},
    },
};

pub fn execute_instruction(
    analyzer_id: &Pubkey,
    context_accounts: &[AccountInfo],
    payload: &[u8],
) -> ProgramResult {
    let decoded = if payload.is_empty() {
        TransactionAnalyzerInstruction::Submit
    } else {
        TransactionAnalyzerInstruction::try_from_slice(payload)
            .map_err(|_| ProgramError::InvalidInstructionData)?
    };

    msg!("{:?}", decoded);

    match decoded {
        TransactionAnalyzerInstruction::Submit => {
            handle_submit_transaction_record(analyzer_id, context_accounts, InstructionMode::Strict)
        }
        TransactionAnalyzerInstruction::SubmitIdempotent => {
            handle_submit_transaction_record(analyzer_id, context_accounts, InstructionMode::Flexible)
        }
        TransactionAnalyzerInstruction::RedirectNested => {
            handle_redirect_nested_record(analyzer_id, context_accounts)
        }
    }
}

