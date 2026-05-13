"use client";

import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { memo, useCallback, useMemo, useState } from "react";
import {
  Activity,
  AlertTriangle,
  ArrowDownRight,
  Check,
  Crosshair,
  Gauge,
  Landmark,
  Loader2,
  Radio,
  Scale,
  UsersRound,
  X,
} from "lucide-react";

import { useMonitor } from "@/hooks/use-monitor";
import { riskOracleApiLabel, type MonitorResponse } from "@/lib/risk-oracle";

const SYMBOL_PRESETS = ["USDC", "USDT", "PYUSD", "SOL"] as const;

function stressFill(score: number): string {
  if (score >= 75) return "#ef4444";
  if (score >= 45) return "#f59e0b";
  return "#3b86b8";
}

/** Separates major blocks: white card on page wash, no borders */
function SectionShell({
  id,
  title,
  description,
  children,
  density = "comfortable",
}: {
  id: string;
  title: string;
  description?: string;
  children: ReactNode;
  density?: "comfortable" | "compact";
}) {
  const pad = density === "compact" ? "px-5 py-5 sm:px-6" : "px-6 py-8 sm:px-8";
  return (
    <section
      aria-labelledby={`${id}-heading`}
      className={[
        "rounded-3xl bg-white/85 shadow-[0_1px_2px_rgba(16,42,58,0.04),0_8px_24px_rgba(16,42,58,0.04)]",
        pad,
      ].join(" ")}
    >
      <header className="mb-8 max-w-3xl">
        <h2
          id={`${id}-heading`}
          className="text-[11px] font-semibold uppercase tracking-[0.28em] text-skyglass-muted"
        >
          {title}
        </h2>
        {description ? (
          <p className="mt-2 text-sm leading-relaxed text-skyglass-muted">{description}</p>
        ) : null}
      </header>
      {children}
    </section>
  );
}

const StressRing = memo(function StressRing({ score, max }: { score: number; max: number }) {
  const r = 88;
  const circumference = 2 * Math.PI * r;
  const pct = Math.min(1, Math.max(0, score / Math.max(1, max)));
  const dashOffset = circumference * (1 - pct);
  const stroke = stressFill(score);

  return (
    <svg
      width="220"
      height="220"
      viewBox="0 0 200 200"
      className="mx-auto block"
      role="img"
      aria-label={`Stress score ${score} out of ${max}`}
    >
      <circle cx="100" cy="100" r={r} fill="none" stroke="rgba(106,176,227,0.22)" strokeWidth="14" />
      <circle
        cx="100"
        cy="100"
        r={r}
        fill="none"
        stroke={stroke}
        strokeWidth="14"
        strokeLinecap="round"
        strokeDasharray={circumference}
        strokeDashoffset={dashOffset}
        transform="rotate(-90 100 100)"
      />
    </svg>
  );
});

function liquidityTone(label: string): "ok" | "warn" | "bad" {
  const s = label.toLowerCase();
  if (s.includes("critical") || s.includes("severe")) return "bad";
  if (s.includes("watch") || s.includes("warning") || s.includes("stressed")) return "warn";
  return "ok";
}

function depegTone(p: string): "ok" | "warn" | "bad" {
  const u = p.toUpperCase();
  if (u === "HIGH" || u === "VERY_HIGH") return "bad";
  if (u === "MEDIUM") return "warn";
  return "ok";
}

function modeTone(mode: string): "ok" | "warn" | "bad" {
  const m = mode.toUpperCase();
  if (m.includes("PANIC")) return "bad";
  if (m.includes("WATCH") || m.includes("RECOVERY")) return "warn";
  return "ok";
}

const toneSurface: Record<"ok" | "warn" | "bad", string> = {
  ok: "bg-skyglass-mist/60",
  warn: "bg-skyglass-ice/50",
  bad: "bg-skyglass-mist/85",
};

const ProtectionRow = memo(function ProtectionRow({ label, active }: { label: string; active: boolean }) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-2xl bg-skyglass-mist/40 px-4 py-3.5">
      <span className="text-sm font-medium text-skyglass-ink">{label}</span>
      <span
        className={[
          "inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-semibold uppercase tracking-wide",
          active ? "bg-skyglass-ice text-navy-800" : "bg-white/80 text-skyglass-muted",
        ].join(" ")}
      >
        {active ? (
          <>
            <Check className="h-3.5 w-3.5 text-skyglass-deep" aria-hidden />
            Active
          </>
        ) : (
          <>
            <X className="h-3.5 w-3.5" aria-hidden />
            Off
          </>
        )}
      </span>
    </div>
  );
});

export default function MonitorDashboard() {
  const [symbol, setSymbol] = useState("USDC");
  const { state, isRefreshing } = useMonitor(symbol);

  const data = state.status === "ok" ? state.data : undefined;
  const errorMessage = state.status === "error" ? state.message : null;

  const isInitialLoad =
    (state.status === "loading" || state.status === "idle") && !data;

  const apiLabel = riskOracleApiLabel();

  const onSymbolClick = useCallback((s: string) => {
    setSymbol(s);
  }, []);

  return (
    <div className="mx-auto w-full max-w-6xl space-y-6">
      <SectionShell
        id="monitor-overview"
        title="Overview"
        description="Stress, liquidity, depeg, breaker posture, and flow context — one screen."
      >
        <div className="space-y-6">
          <div className="flex flex-wrap items-center gap-2 font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
            <Radio className="h-3.5 w-3.5 shrink-0" aria-hidden />
            <span>Risk oracle feed</span>
          </div>

          <div>
            <h1 className="text-3xl font-semibold tracking-tight text-skyglass-ink md:text-4xl">Live monitor</h1>
            <p className="mt-3 max-w-xl text-sm leading-relaxed text-skyglass-muted">
              <span className="font-mono text-xs text-skyglass-ink/80">{apiLabel}</span>
              {isRefreshing && data ? (
                <span className="ml-2 inline-flex items-center gap-1">
                  <Loader2 className="h-3 w-3 animate-spin" aria-hidden />
                  <span className="text-skyglass-muted">syncing</span>
                </span>
              ) : null}
            </p>
          </div>

          <div className="flex flex-col gap-6 pt-2 sm:flex-row sm:items-end sm:justify-between">
            <div className="space-y-3">
              <span className="text-xs font-semibold uppercase tracking-wide text-skyglass-muted">Symbol</span>
              <div className="flex flex-wrap gap-2">
                {SYMBOL_PRESETS.map((s) => (
                  <button
                    key={s}
                    type="button"
                    onClick={() => onSymbolClick(s)}
                    className={[
                      "rounded-full px-4 py-2 text-sm font-medium",
                      symbol === s
                        ? "bg-skyglass-deep text-white"
                        : "bg-skyglass-mist/90 text-skyglass-muted",
                    ].join(" ")}
                  >
                    {s}
                  </button>
                ))}
              </div>
            </div>
            <div className="w-full sm:max-w-xs">
              <label htmlFor="symbol-custom" className="sr-only">
                Custom symbol
              </label>
              <input
                id="symbol-custom"
                value={symbol}
                onChange={(e) => setSymbol(e.target.value.toUpperCase())}
                className="w-full rounded-2xl bg-skyglass-mist/50 px-4 py-2.5 font-mono text-sm text-skyglass-ink outline-none placeholder:text-skyglass-muted focus-visible:bg-skyglass-ice/40 focus-visible:ring-2 focus-visible:ring-skyglass-deep/20"
                placeholder="Custom symbol"
                autoComplete="off"
              />
            </div>
          </div>

          {data?.data_source?.startsWith("demo") ? (
            <p className="rounded-2xl bg-skyglass-mist/40 px-4 py-3 text-sm text-skyglass-muted">
              <span className="font-medium text-skyglass-ink">Demo stream</span> — ticks with server time. For
              on-chain data, set{" "}
              <code className="rounded bg-white/90 px-1.5 py-0.5 font-mono text-xs text-skyglass-ink">
                RISK_ORACLE_MONITOR_*
              </code>{" "}
              on the API.
            </p>
          ) : null}

          {errorMessage ? (
            <div role="alert" className="flex gap-3 rounded-2xl bg-red-50/85 p-4 text-sm text-red-900">
              <AlertTriangle className="mt-0.5 h-5 w-5 shrink-0" aria-hidden />
              <div>
                <p className="font-semibold">API unreachable</p>
                <p className="mt-1 text-red-800/90">
                  {errorMessage}. Run <code className="font-mono text-xs">risk-oracle-api</code> or fix env / proxy.
                </p>
              </div>
            </div>
          ) : null}

          {isInitialLoad ? (
            <div className="flex items-center gap-2 rounded-2xl bg-skyglass-mist/40 px-4 py-6 text-skyglass-muted">
              <Loader2 className="h-5 w-5 animate-spin shrink-0" aria-hidden />
              Loading…
            </div>
          ) : null}
        </div>
      </SectionShell>

      {data ? <MonitorPanels data={data} /> : null}
    </div>
  );
}

const MonitorPanels = memo(function MonitorPanels({ data }: { data: MonitorResponse }) {
  const liq = liquidityTone(data.core.liquidity_health);
  const dep = depegTone(data.core.depeg_probability);
  const mod = modeTone(data.circuit_breaker.protection_mode);
  const updated = useMemo(() => new Date(data.updated_at_unix_ms).toLocaleString(), [data.updated_at_unix_ms]);

  const secondaryItems = useMemo(
    () =>
      [
        {
          key: "velocity",
          Icon: ArrowDownRight,
          title: "Withdrawal velocity",
          value: data.secondary.withdrawal_velocity,
        },
        {
          key: "imbalance",
          Icon: Scale,
          title: "Pool imbalance",
          value: data.secondary.pool_imbalance,
        },
        {
          key: "whale",
          Icon: UsersRound,
          title: "Whale exit activity",
          value:
            data.secondary.whale_abnormal_exits === 0
              ? "No abnormal whale exits detected"
              : `${data.secondary.whale_abnormal_exits} abnormal whale exits detected`,
        },
        {
          key: "oracle",
          Icon: Crosshair,
          title: "Oracle divergence",
          value: `${data.secondary.oracle_divergence_bps} bps deviation`,
        },
        {
          key: "bridge",
          Icon: Landmark,
          title: "Bridge outflows",
          value: data.secondary.bridge_outflows,
        },
      ] as const,
    [data.secondary],
  );

  return (
    <div className="space-y-6">
      <SectionShell
        id="primary-metrics"
        title="Primary"
        description="Stress ring plus liquidity, depeg, and active protection mode."
      >
        <div className="grid gap-8 lg:grid-cols-12 lg:gap-10">
          <div className="lg:col-span-5">
            <div className="rounded-2xl bg-skyglass-mist/35 px-5 py-8 sm:px-6">
              <div className="mb-4 flex items-center gap-2 text-xs font-semibold uppercase tracking-wide text-skyglass-muted">
                <Gauge className="h-4 w-4 text-skyglass-deep" aria-hidden />
                Stress score
              </div>
              <div className="relative mx-auto h-[220px] w-[220px]">
                <StressRing score={data.core.stress_score} max={data.core.stress_max} />
                <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center pt-4">
                  <div className="text-5xl font-bold tabular-nums tracking-tight text-skyglass-ink">
                    {data.core.stress_score}
                  </div>
                  <div className="text-sm font-medium text-skyglass-muted">/ {data.core.stress_max}</div>
                  <p className="mt-2 text-center text-xs text-skyglass-muted">0–100 stress index</p>
                </div>
              </div>
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2 lg:col-span-7 lg:content-start">
            <MetricCard title="Liquidity health" subtitle="Pool depth & exits" tone={liq}>
              <p className="text-2xl font-semibold tracking-tight text-skyglass-ink">{data.core.liquidity_health}</p>
            </MetricCard>

            <MetricCard title="Depeg probability" subtitle="Near-term peg risk" tone={dep}>
              <p
                className={[
                  "text-2xl font-bold tracking-wide",
                  dep === "bad" ? "text-status-error" : dep === "warn" ? "text-status-warn" : "text-status-ok",
                ].join(" ")}
              >
                {data.core.depeg_probability}
              </p>
            </MetricCard>

            <MetricCard
              title="Protection mode"
              subtitle="Circuit breaker posture"
              tone={mod}
              className="sm:col-span-2"
            >
              <p className="text-xl font-semibold tracking-tight text-skyglass-ink md:text-2xl">
                {data.circuit_breaker.protection_mode}
              </p>
            </MetricCard>
          </div>
        </div>
      </SectionShell>

      <SectionShell
        id="active-protections"
        title="Protections"
        description="Which rails are engaged right now."
      >
        <div className="grid gap-3 sm:grid-cols-2">
          <ProtectionRow label="Dynamic fees enabled" active={data.circuit_breaker.dynamic_fees_enabled} />
          <ProtectionRow label="Withdrawal throttling active" active={data.circuit_breaker.withdrawal_throttling_active} />
          <ProtectionRow label="Liquidity routing active" active={data.circuit_breaker.liquidity_routing_active} />
          <ProtectionRow label="Toxic routing restricted" active={data.circuit_breaker.toxic_routing_restricted} />
        </div>
      </SectionShell>

      <SectionShell
        id="secondary-metrics"
        title="Secondary"
        description="Flow and market shape behind the headline score."
        density="compact"
      >
        <ul className="grid list-none gap-3 p-0 sm:grid-cols-2 xl:grid-cols-3">
          {secondaryItems.map((row) => (
            <SecondaryRow key={row.key} Icon={row.Icon} title={row.title} value={row.value} />
          ))}
        </ul>

        <div className="mt-8 flex flex-wrap items-center gap-x-5 gap-y-2 pt-2 text-xs text-skyglass-muted">
          <span className="rounded-full bg-skyglass-mist/60 px-3 py-1.5 font-mono text-[10px] uppercase tracking-widest text-skyglass-ink">
            Source {data.data_source}
          </span>
          <span>
            <span className="text-skyglass-muted">Symbol</span>{" "}
            <span className="font-mono font-semibold text-skyglass-ink">{data.symbol}</span>
          </span>
          <span>
            <span className="text-skyglass-muted">Updated</span>{" "}
            <time
              className="font-mono text-skyglass-ink/90"
              dateTime={new Date(data.updated_at_unix_ms).toISOString()}
            >
              {updated}
            </time>
          </span>
        </div>
      </SectionShell>
    </div>
  );
});

const SecondaryRow = memo(function SecondaryRow({
  Icon,
  title,
  value,
}: {
  Icon: LucideIcon;
  title: string;
  value: string;
}) {
  return (
    <li className="rounded-2xl bg-skyglass-mist/35 px-4 py-4">
      <div className="flex gap-3">
        <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-white/70 text-skyglass-deep">
          <Icon className="h-5 w-5" strokeWidth={1.75} aria-hidden />
        </div>
        <div className="min-w-0 flex-1">
          <p className="text-[10px] font-semibold uppercase tracking-[0.18em] text-skyglass-muted">{title}</p>
          <p className="mt-1.5 text-sm font-medium leading-snug text-navy-800">{value}</p>
        </div>
      </div>
    </li>
  );
});

const MetricCard = memo(function MetricCard({
  title,
  subtitle,
  tone,
  className,
  children,
}: {
  title: string;
  subtitle: string;
  tone: "ok" | "warn" | "bad";
  className?: string;
  children: ReactNode;
}) {
  return (
    <div className={["rounded-2xl p-5", toneSurface[tone], className ?? ""].join(" ")}>
      <div className="font-mono text-[10px] uppercase tracking-[0.2em] text-skyglass-muted">{title}</div>
      <div className="mt-1 text-xs text-skyglass-muted">{subtitle}</div>
      <div className="mt-4">{children}</div>
    </div>
  );
});
