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
    },
}

impl OracleInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        
        match tag {
            0 => Ok(Self::Initialize),
            1 => {
                if rest.len() < 3 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                Ok(Self::UpdateRisk {
                    stress_score: rest[0],
                    liquidity_health: rest[1],
                    depeg_probability: rest[2],
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
        let payload = [1, 85, 3, 2]; // tag, score, health, depeg
        let instruction = OracleInstruction::unpack(&payload).unwrap();
        match instruction {
            OracleInstruction::UpdateRisk {
                stress_score,
                liquidity_health,
                depeg_probability,
            } => {
                assert_eq!(stress_score, 85);
                assert_eq!(liquidity_health, 3);
                assert_eq!(depeg_probability, 2);
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
