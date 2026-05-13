use crate::error::CircuitBreakerError;
use crate::state::ProtectionMode;

pub enum CircuitBreakerInstruction {
    Initialize,
    UpdatePolicy {
        mode: ProtectionMode,
        adaptive_fee_bps: u16,
        withdrawal_throttle_pct: u8,
        toxic_routing_restricted: bool,
        current_slot: u64,
    },
}

impl CircuitBreakerInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, CircuitBreakerError> {
        let (&tag, rest) = input.split_first().ok_or(CircuitBreakerError::InvalidInstruction)?;
        
        match tag {
            0 => Ok(Self::Initialize),
            1 => {
                if rest.len() < 13 {
                    return Err(CircuitBreakerError::InvalidInstruction);
                }
                let mode = ProtectionMode::try_from(rest[0])?;
                
                let mut fee_bytes = [0u8; 2];
                fee_bytes.copy_from_slice(&rest[1..3]);
                let adaptive_fee_bps = u16::from_le_bytes(fee_bytes);
                
                let withdrawal_throttle_pct = rest[3];
                let toxic_routing_restricted = rest[4] != 0;
                
                let mut slot_bytes = [0u8; 8];
                slot_bytes.copy_from_slice(&rest[5..13]);
                let current_slot = u64::from_le_bytes(slot_bytes);
                
                Ok(Self::UpdatePolicy {
                    mode,
                    adaptive_fee_bps,
                    withdrawal_throttle_pct,
                    toxic_routing_restricted,
                    current_slot,
                })
            }
            _ => Err(CircuitBreakerError::InvalidInstruction),
        }
    }
}
