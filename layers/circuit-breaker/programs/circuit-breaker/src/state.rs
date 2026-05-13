use crate::error::CircuitBreakerError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ProtectionMode {
    Normal = 0,
    Watch = 1,
    PanicProtection = 2,
    Recovery = 3,
}

impl TryFrom<u8> for ProtectionMode {
    type Error = CircuitBreakerError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(ProtectionMode::Normal),
            1 => Ok(ProtectionMode::Watch),
            2 => Ok(ProtectionMode::PanicProtection),
            3 => Ok(ProtectionMode::Recovery),
            _ => Err(CircuitBreakerError::InvalidStateData),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CircuitBreakerState {
    pub authority: [u8; 32],
    pub mode: ProtectionMode,
    pub adaptive_fee_bps: u16,
    pub withdrawal_throttle_pct: u8,
    pub toxic_routing_restricted: bool,
    pub last_updated_slot: u64,
}

impl CircuitBreakerState {
    pub const LEN: usize = 32 + 1 + 2 + 1 + 1 + 8; // 45 bytes

    pub fn unpack(data: &[u8]) -> Result<Self, CircuitBreakerError> {
        if data.len() < Self::LEN {
            return Err(CircuitBreakerError::InvalidStateData);
        }
        
        let mut authority = [0u8; 32];
        authority.copy_from_slice(&data[0..32]);
        
        let mode = ProtectionMode::try_from(data[32])?;
        
        let mut fee_bytes = [0u8; 2];
        fee_bytes.copy_from_slice(&data[33..35]);
        let adaptive_fee_bps = u16::from_le_bytes(fee_bytes);
        
        let withdrawal_throttle_pct = data[35];
        let toxic_routing_restricted = data[36] != 0;
        
        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&data[37..45]);
        let last_updated_slot = u64::from_le_bytes(slot_bytes);
        
        Ok(Self {
            authority,
            mode,
            adaptive_fee_bps,
            withdrawal_throttle_pct,
            toxic_routing_restricted,
            last_updated_slot,
        })
    }

    pub fn pack(&self, data: &mut [u8]) -> Result<(), CircuitBreakerError> {
        if data.len() < Self::LEN {
            return Err(CircuitBreakerError::InvalidStateData);
        }
        
        data[0..32].copy_from_slice(&self.authority);
        data[32] = self.mode as u8;
        data[33..35].copy_from_slice(&self.adaptive_fee_bps.to_le_bytes());
        data[35] = self.withdrawal_throttle_pct;
        data[36] = if self.toxic_routing_restricted { 1 } else { 0 };
        data[37..45].copy_from_slice(&self.last_updated_slot.to_le_bytes());
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack() {
        let mut data = [0u8; CircuitBreakerState::LEN];
        let state = CircuitBreakerState {
            authority: [1u8; 32],
            mode: ProtectionMode::PanicProtection,
            adaptive_fee_bps: 500,
            withdrawal_throttle_pct: 10,
            toxic_routing_restricted: true,
            last_updated_slot: 123456789,
        };

        state.pack(&mut data).unwrap();

        let unpacked = CircuitBreakerState::unpack(&data).unwrap();
        
        assert_eq!(unpacked.authority, state.authority);
        assert_eq!(unpacked.mode, state.mode);
        assert_eq!(unpacked.adaptive_fee_bps, state.adaptive_fee_bps);
        assert_eq!(unpacked.withdrawal_throttle_pct, state.withdrawal_throttle_pct);
        assert_eq!(unpacked.toxic_routing_restricted, state.toxic_routing_restricted);
        assert_eq!(unpacked.last_updated_slot, state.last_updated_slot);
    }

    #[test]
    fn test_invalid_state_data_length() {
        let mut short_data = [0u8; CircuitBreakerState::LEN - 1];
        let state = CircuitBreakerState {
            authority: [1u8; 32],
            mode: ProtectionMode::Normal,
            adaptive_fee_bps: 0,
            withdrawal_throttle_pct: 100,
            toxic_routing_restricted: false,
            last_updated_slot: 0,
        };

        assert!(state.pack(&mut short_data).is_err());
        assert!(CircuitBreakerState::unpack(&short_data).is_err());
    }
}
