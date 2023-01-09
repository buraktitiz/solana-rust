pub mod instruction;
use instruction::{MovieInstruction};

use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    
    let instruction = MovieInstruction::unpack(instruction_data)?;
   
    match instruction {
        MovieInstruction::AddMovieReview { title, rating, description } => {         
            add_movie_review(program_id, accounts, title, rating, description)
        }
    }
}

pub fn add_movie_review(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    title: String,
    rating: u8,
    _description: String
) -> ProgramResult {

    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", _description);

    Ok(())
}