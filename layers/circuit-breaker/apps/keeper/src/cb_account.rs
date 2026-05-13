use shock_absorber_circuit_breaker::types::ProtectionMode;

use crate::error::KeeperError;

/// Serialized size of the circuit breaker control account (`CircuitBreakerState::LEN`).
pub const CIRCUIT_BREAKER_STATE_LEN: usize = 45;

#[derive(Debug, Clone, Copy)]
pub struct CircuitBreakerAccountHead {
    pub mode: ProtectionMode,
}

pub fn decode_circuit_breaker_state(data: &[u8]) -> Result<CircuitBreakerAccountHead, KeeperError> {
    if data.len() < CIRCUIT_BREAKER_STATE_LEN {
        return Err(KeeperError::CircuitBreaker(format!(
            "account data too short: {} (need {CIRCUIT_BREAKER_STATE_LEN})",
            data.len()
        )));
    }

    let mode = ProtectionMode::try_from_discriminant(data[32]).ok_or_else(|| {
        KeeperError::CircuitBreaker(format!("invalid protection mode byte {}", data[32]))
    })?;

    Ok(CircuitBreakerAccountHead { mode })
}
