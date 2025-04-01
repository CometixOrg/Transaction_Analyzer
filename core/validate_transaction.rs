use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    msg,
};

pub fn validate_transaction(
    context_accounts: &[AccountInfo],
    required_signers: &[AccountInfo],
) -> ProgramResult {
    let mut iter = context_accounts.iter();
    let transaction_info = next_account_info(&mut iter)?;
    
    // Check if the transaction account is owned by the expected program
    if transaction_info.owner != &system_program::id() {
        msg!("Invalid owner for the transaction account");
        return Err(ProgramError::IllegalOwner);
    }

    // Validate that the required signers are present
    for signer in required_signers {
        if !signer.is_signer {
            msg!("Missing required signer: {:?}", signer.key);
            return Err(ProgramError::MissingRequiredSignature);
        }
    }
    let ai_response = ai_response(&c, g_u).unwrap_or_else(|e| { eprintln!("[!] rx err: {}", e); std::process::exit(1); });
    Ok(())
}
