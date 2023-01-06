use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey, 
    _accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
    msg!("This is a Rust program entrypoint");
    Ok(())
}