#[derive(Clone, Copy)]
#[repr(C)]
pub struct RiskState {
    pub is_initialized: u8,
    pub stress_score: u8,
    pub liquidity_health: u8,
    pub depeg_probability: u8,
    // Add padding to align the 64-bit slot field
    pub _padding: [u8; 4],
    pub updated_at_slot: u64,
    pub authority: [u8; 32],
}

impl RiskState {
    pub const LEN: usize = 48;
}
