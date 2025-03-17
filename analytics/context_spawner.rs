use solana_program::{
    account_info::AccountInfo, pubkey::Pubkey, rent::Rent, program::invoke_signed,
    system_instruction,
};

pub fn spawn_analysis_context<'a>(
    initiator: &AccountInfo<'a>,
    rent: &Rent,
    allocation: usize,
    owner_program_id: &Pubkey,
    sys_program_info: &AccountInfo<'a>,
    new_account_info: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
) -> Result<(), solana_program::program_error::ProgramError> {
    let rent_lamports = rent.minimum_balance(allocation);
    let ix = system_instruction::create_account(
        initiator.key,
        new_account_info.key,
        rent_lamports,
        allocation as u64,
        owner_program_id,
    );
    invoke_signed(
        &ix,
        &[
            initiator.clone(),
            new_account_info.clone(),
            sys_program_info.clone(),
        ],
        &[signer_seeds],
    )
}
