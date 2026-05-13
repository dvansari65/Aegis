//! Integration tests: risk oracle account bytes → circuit breaker policy (no Solana validator).

use circuit_breaker_keeper::ix;
use circuit_breaker_keeper::pipeline::{decode_update_policy_ix, run_policy_tick_from_accounts};
use shock_absorber_circuit_breaker::types::ProtectionMode;

#[test]
fn ix_encoder_matches_program_golden_vector() {
    let bytes = ix::encode_update_policy(ProtectionMode::Normal, 5, 100, false, 0);
    assert_eq!(
        bytes,
        vec![1, 0, 5, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn pipeline_integration_watch_mode_from_stress() {
    let mut risk = [0u8; 48];
    risk[0] = 1;
    risk[1] = 60;
    risk[2] = 0;
    risk[3] = 0;
    risk[8..16].copy_from_slice(&500u64.to_le_bytes());

    let mut cb = [0u8; 45];
    cb[32] = ProtectionMode::Normal as u8;

    let out = run_policy_tick_from_accounts(1_200, &risk, &cb, 90).expect("tick");
    assert_eq!(out.wire.mode, ProtectionMode::Watch);

    let raw: Vec<u8> = (0..out.wire.update_policy_instruction_hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&out.wire.update_policy_instruction_hex[i..i + 2], 16).unwrap()
        })
        .collect();

    let decoded = decode_update_policy_ix(&raw).expect("decode");
    assert_eq!(decoded.0, out.wire.mode as u8);
}
