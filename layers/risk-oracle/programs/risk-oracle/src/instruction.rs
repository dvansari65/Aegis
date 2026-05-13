use pinocchio::error::ProgramError;

pub enum OracleInstruction {
    /// Initialize the risk oracle state
    /// Accounts:
    /// 0. `[writable]` Risk state account
    /// 1. `[signer]` Authority
    Initialize,
    
    /// Update risk score
    /// Accounts:
    /// 0. `[writable]` Risk state account
    /// 1. `[signer]` Authority
    UpdateRisk {
        stress_score: u8,
        liquidity_health: u8,
        depeg_probability: u8,
        /// When `Some`, written to `RiskState.updated_at_slot` (little-endian).
        /// Legacy 3-byte payloads omit this and leave the slot unchanged.
        updated_at_slot: Option<u64>,
    },
}

impl OracleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        match tag {
            0 => Ok(Self::Initialize),
            1 => {
                let (stress_score, liquidity_health, depeg_probability, updated_at_slot) =
                    match rest.len() {
                        3 => (
                            rest[0],
                            rest[1],
                            rest[2],
                            None,
                        ),
                        n if n >= 11 => {
                            let mut slot_bytes = [0u8; 8];
                            slot_bytes.copy_from_slice(&rest[3..11]);
                            (
                                rest[0],
                                rest[1],
                                rest[2],
                                Some(u64::from_le_bytes(slot_bytes)),
                            )
                        }
                        _ => return Err(ProgramError::InvalidInstructionData),
                    };

                Ok(Self::UpdateRisk {
                    stress_score,
                    liquidity_health,
                    depeg_probability,
                    updated_at_slot,
                })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_initialize() {
        let payload = [0];
        let instruction = OracleInstruction::unpack(&payload).unwrap();
        match instruction {
            OracleInstruction::Initialize => {}
            _ => panic!("Wrong instruction unpacked"),
        }
    }

    #[test]
    fn test_unpack_update_risk() {
        let mut payload = [0u8; 12];
        payload[0..4].copy_from_slice(&[1, 85, 3, 2]);
        payload[4..12].copy_from_slice(&9_999u64.to_le_bytes());

        let instruction = OracleInstruction::unpack(&payload).unwrap();
        match instruction {
            OracleInstruction::UpdateRisk {
                stress_score,
                liquidity_health,
                depeg_probability,
                updated_at_slot,
            } => {
                assert_eq!(stress_score, 85);
                assert_eq!(liquidity_health, 3);
                assert_eq!(depeg_probability, 2);
                assert_eq!(updated_at_slot, Some(9_999));
            }
            _ => panic!("Wrong instruction unpacked"),
        }
    }

    #[test]
    fn test_unpack_update_risk_legacy_no_slot() {
        let payload = [1, 85, 3, 2];
        let instruction = OracleInstruction::unpack(&payload).unwrap();
        match instruction {
            OracleInstruction::UpdateRisk {
                stress_score,
                liquidity_health,
                depeg_probability,
                updated_at_slot,
            } => {
                assert_eq!(stress_score, 85);
                assert_eq!(liquidity_health, 3);
                assert_eq!(depeg_probability, 2);
                assert!(updated_at_slot.is_none());
            }
            _ => panic!("Wrong instruction unpacked"),
        }
    }

    #[test]
    fn test_unpack_invalid_tag() {
        let payload = [2];
        assert!(OracleInstruction::unpack(&payload).is_err());
    }

    #[test]
    fn test_unpack_update_risk_too_short() {
        let payload = [1, 85, 3]; // Missing depeg_probability
        assert!(OracleInstruction::unpack(&payload).is_err());
    }
}
