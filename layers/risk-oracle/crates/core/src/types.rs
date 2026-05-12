use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidityHealth {
    Healthy,
    Watch,
    Stressed,
    Critical,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepegProbabilityBand {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablecoinId {
    pub symbol: String,
    pub mint: String,
}
