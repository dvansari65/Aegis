export type CoreMetrics = {
  stress_score: number;
  stress_max: number;
  liquidity_health: string;
  depeg_probability: string;
};

export type CircuitBreakerView = {
  protection_mode: string;
  dynamic_fees_enabled: boolean;
  withdrawal_throttling_active: boolean;
  liquidity_routing_active: boolean;
  toxic_routing_restricted: boolean;
};

export type SecondarySignals = {
  withdrawal_velocity: string;
  pool_imbalance: string;
  whale_abnormal_exits: number;
  oracle_divergence_bps: number;
  bridge_outflows: string;
};

export type MonitorResponse = {
  symbol: string;
  updated_at_unix_ms: number;
  /** `demo-live` = clock-synced synthetic; `solana` = risk oracle account + policy engines */
  data_source: string;
  core: CoreMetrics;
  circuit_breaker: CircuitBreakerView;
  secondary: SecondarySignals;
};

/** Base URL when the browser calls the API directly (must match CORS + reachable host). */
export function riskOracleApiBase(): string {
  const raw = process.env.NEXT_PUBLIC_RISK_ORACLE_API_URL?.trim();
  if (raw) return raw.replace(/\/$/, "");
  return "";
}

/** Human-readable line for the monitor header. */
export function riskOracleApiLabel(): string {
  const raw = process.env.NEXT_PUBLIC_RISK_ORACLE_API_URL?.trim();
  if (raw) return raw.replace(/\/$/, "");
  return "Same-origin /risk-oracle-api → Rust API (set RISK_ORACLE_API_URL in .env.local for the Next server)";
}

/**
 * When `NEXT_PUBLIC_RISK_ORACLE_API_URL` is unset, use the Next.js rewrite
 * `/risk-oracle-api/*` so the browser never talks to a different origin (fixes typical `Failed to fetch`).
 */
export function monitorRequestUrl(symbol: string): string {
  const sym = encodeURIComponent(symbol.trim() || "USDC");
  const base = riskOracleApiBase();
  if (base) return `${base}/v1/risk/${sym}`;
  return `/risk-oracle-api/v1/risk/${sym}`;
}

export async function fetchMonitor(symbol: string, signal?: AbortSignal): Promise<MonitorResponse> {
  const url = monitorRequestUrl(symbol);
  const res = await fetch(url, { cache: "no-store", signal });
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(text || `HTTP ${res.status}`);
  }
  return res.json() as Promise<MonitorResponse>;
}
