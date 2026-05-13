#![no_std]

pub mod error;
pub mod instruction;
pub mod state;

use pinocchio::{
    no_allocator, nostd_panic_handler, program_entrypoint, AccountView, Address, ProgramResult,
};
use crate::{
    error::CircuitBreakerError,
    instruction::CircuitBreakerInstruction,
    state::CircuitBreakerState,
};

program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

// Map CircuitBreakerError to pinocchio ProgramError
fn map_err(e: CircuitBreakerError) -> pinocchio::error::ProgramError {
    pinocchio::error::ProgramError::Custom(e as u32)
}

pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CircuitBreakerInstruction::unpack(instruction_data)
        .map_err(map_err)?;

    match instruction {
        CircuitBreakerInstruction::Initialize => process_initialize(accounts),
        CircuitBreakerInstruction::UpdatePolicy {
            mode,
            adaptive_fee_bps,
            withdrawal_throttle_pct,
            toxic_routing_restricted,
            current_slot,
        } => process_update_policy(
            accounts,
            mode,
            adaptive_fee_bps,
            withdrawal_throttle_pct,
            toxic_routing_restricted,
            current_slot,
        ),
    }
}

fn process_initialize(accounts: &mut [AccountView]) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(map_err(CircuitBreakerError::NotEnoughAccounts));
    }

    let (left, right) = accounts.split_at_mut(1);
    let authority = &left[0];
    let state_account = &mut right[0];

    if !authority.is_signer() {
        return Err(map_err(CircuitBreakerError::Unauthorized));
    }

    let mut state_data = state_account.try_borrow_mut()?;

    let mut authority_bytes = [0u8; 32];
    authority_bytes.copy_from_slice(authority.address().as_ref());

    let state = CircuitBreakerState {
        authority: authority_bytes,
        mode: state::ProtectionMode::Normal,
        adaptive_fee_bps: 0,
        withdrawal_throttle_pct: 100,
        toxic_routing_restricted: false,
        last_updated_slot: 0,
    };

    state.pack(&mut state_data).map_err(map_err)?;

    Ok(())
}

fn process_update_policy(
    accounts: &mut [AccountView],
    mode: state::ProtectionMode,
    adaptive_fee_bps: u16,
    withdrawal_throttle_pct: u8,
    toxic_routing_restricted: bool,
    current_slot: u64,
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(map_err(CircuitBreakerError::NotEnoughAccounts));
    }

    let (left, right) = accounts.split_at_mut(1);
    let authority = &left[0];
    let state_account = &mut right[0];

    if !authority.is_signer() {
        return Err(map_err(CircuitBreakerError::Unauthorized));
    }

    let mut state_data = state_account.try_borrow_mut()?;
    let mut state = CircuitBreakerState::unpack(&state_data).map_err(map_err)?;

    let mut authority_bytes = [0u8; 32];
    authority_bytes.copy_from_slice(authority.address().as_ref());

    if state.authority != authority_bytes {
        return Err(map_err(CircuitBreakerError::Unauthorized));
    }

    state.mode = mode;
    state.adaptive_fee_bps = adaptive_fee_bps;
    state.withdrawal_throttle_pct = withdrawal_throttle_pct;
    state.toxic_routing_restricted = toxic_routing_restricted;
    state.last_updated_slot = current_slot;

    state.pack(&mut state_data).map_err(map_err)?;

    Ok(())
}
