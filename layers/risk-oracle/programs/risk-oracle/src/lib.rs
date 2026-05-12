#![no_std]

pub mod instruction;
pub mod state;

use pinocchio::{
    no_allocator, nostd_panic_handler, program_entrypoint, AccountView, Address, ProgramResult, error::ProgramError,
};
use instruction::OracleInstruction;
use state::RiskState;

program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = OracleInstruction::unpack(instruction_data)?;

    match instruction {
        OracleInstruction::Initialize => {
            if accounts.len() < 2 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            
            let (state_slice, rest) = accounts.split_at_mut(1);
            let state_account = &mut state_slice[0];
            let authority_account = &rest[0];

            if !authority_account.is_signer() {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let mut state_data = state_account.try_borrow_mut().map_err(|_| ProgramError::AccountBorrowFailed)?;
            
            if state_data.len() < RiskState::LEN {
                return Err(ProgramError::AccountDataTooSmall);
            }

            if state_data[0] != 0 {
                return Err(ProgramError::Custom(2)); // Account already initialized
            }

            state_data[0] = 1; // is_initialized
            // Write authority (offset 16 bytes: 1+1+1+1+4+8 = 16)
            state_data[16..48].copy_from_slice(authority_account.address().as_ref());
            
            Ok(())
        }
        OracleInstruction::UpdateRisk {
            stress_score,
            liquidity_health,
            depeg_probability,
        } => {
            if accounts.len() < 2 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }
            
            let (state_slice, rest) = accounts.split_at_mut(1);
            let state_account = &mut state_slice[0];
            let authority_account = &rest[0];

            if !authority_account.is_signer() {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let mut state_data = state_account.try_borrow_mut().map_err(|_| ProgramError::AccountBorrowFailed)?;
            
            if state_data.len() < RiskState::LEN {
                return Err(ProgramError::AccountDataTooSmall);
            }

            if state_data[0] == 0 {
                return Err(ProgramError::UninitializedAccount);
            }

            if &state_data[16..48] != authority_account.address().as_ref() {
                return Err(ProgramError::Custom(1)); // Unauthorized
            }

            state_data[1] = stress_score;
            state_data[2] = liquidity_health;
            state_data[3] = depeg_probability;
            
            Ok(())
        }
    }
}
