use shock_absorber_circuit_breaker::types::ProtectionMode;

/// Encode `CircuitBreakerInstruction::UpdatePolicy` (tag = 1) for the circuit breaker program.
pub fn encode_update_policy(
    mode: ProtectionMode,
    adaptive_fee_bps: u16,
    withdrawal_throttle_pct: u8,
    toxic_routing_restricted: bool,
    current_slot: u64,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(14);
    out.push(1);
    out.push(mode as u8);
    out.extend_from_slice(&adaptive_fee_bps.to_le_bytes());
    out.push(withdrawal_throttle_pct);
    out.push(u8::from(toxic_routing_restricted));
    out.extend_from_slice(&current_slot.to_le_bytes());
    out
}
