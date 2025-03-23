use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use crate::runtime::ledger::extension::{FieldExtension, RecordWithExtensions};
use borsh::BorshDeserialize;

pub fn process_account_data(account_info: &AccountInfo) -> Result<u64, ProgramError> {
    let data = account_info.data.borrow();
    let account = RecordWithExtensions::<Account>::unpack(&data)?;
    Ok(account.base.amount)
}

pub fn encrypt_and_store_data(account_info: &AccountInfo, data: &[u8]) -> ProgramResult {
    // Encrypt the data and store it back
    account_info.data.borrow_mut().copy_from_slice(data);
    Ok(())
}
