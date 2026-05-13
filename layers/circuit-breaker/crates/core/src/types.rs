use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectionMode {
    Normal = 0,
    Watch = 1,
    PanicProtection = 2,
    Recovery = 3,
}

impl ProtectionMode {
    /// Discriminant stored by the on-chain circuit breaker program (`u8`).
    pub fn try_from_discriminant(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Normal),
            1 => Some(Self::Watch),
            2 => Some(Self::PanicProtection),
            3 => Some(Self::Recovery),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlAction {
    NoAction,
    IncreaseFees,
    ThrottleWithdrawals,
    RebalanceLiquidity,
    RestrictToxicRoutes,
    CoordinateProtocolResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidityHealth {
    Healthy,
    Watch,
    Stressed,
    Critical,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepegProbability {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanicType {
    None,
    LiquidityPanic,
    OracleDislocation,
    WhaleExit,
    BridgeOutflow,
    SystemicMarketStress,
    PossibleManipulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionReason {
    HealthyMarket,
    LowConfidence,
    WatchThresholdBreached,
    PanicThresholdBreached,
    SevereLiquidityStress,
    HighDepegRisk,
    RecoveryConditionsMet,
    RecoveryStillCoolingDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryReason {
    NotInEmergencyMode,
    StressStillElevated,
    LiquidityNotRecovered,
    DepegRiskStillElevated,
    PanicTypeStillActive,
    CooldownRemaining,
    ConditionsMet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskOracleSnapshot {
    pub stress_score: u8,
    pub liquidity_health: LiquidityHealth,
    pub depeg_probability: DepegProbability,
    pub panic_type: PanicType,
    pub confidence: u8,
    pub last_updated_slot: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub previous_mode: ProtectionMode,
    pub current_slot: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitBreakerDecision {
    pub mode: ProtectionMode,
    pub reason: DecisionReason,
    pub actions: Vec<ControlAction>,
    pub snapshot: RiskOracleSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveFeePolicy {
    pub fee_bps: u16,
    pub max_fee_bps: u16,
    pub mode: ProtectionMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDecision {
    pub can_exit_panic: bool,
    pub next_mode: ProtectionMode,
    pub cooldown_remaining_slots: u64,
    pub reason: RecoveryReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawalThrottlePolicy {
    pub mode: ProtectionMode,
    pub max_withdrawal_bps_per_window: u16,
    pub window_slots: u64,
    pub delay_slots: u64,
    pub queue_large_withdrawals: bool,
    pub large_withdrawal_threshold_bps: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingMode {
    Normal,
    PreferDeepLiquidity,
    SplitAcrossDeepRoutes,
    AvoidStressedPools,
    EmergencyOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiquidityRoutingPolicy {
    pub mode: ProtectionMode,
    pub routing_mode: RoutingMode,
    pub min_liquidity_depth_bps: u16,
    pub max_single_route_share_bps: u16,
    pub allow_stressed_pools: bool,
    pub require_oracle_consistency: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToxicArbitrageLevel {
    Off,
    Observe,
    Guarded,
    Restricted,
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToxicArbitragePolicy {
    pub mode: ProtectionMode,
    pub level: ToxicArbitrageLevel,
    pub execution_delay_slots: u64,
    pub max_route_imbalance_bps: u16,
    pub restrict_same_slot_round_trips: bool,
    pub require_mev_protected_path: bool,
}
