import {
  Activity,
  ArrowRight,
  CircuitBoard,
  Repeat,
  ShieldHalf,
  SlidersHorizontal,
  Timer,
  Waves,
  Wallet,
  Radio,
  Gauge,
  ArrowDownToLine,
  Zap,
} from "lucide-react";

const monitoredSignals = [
  { icon: ArrowDownToLine, label: "Withdrawal velocity" },
  { icon: Waves,           label: "Liquidity imbalance" },
  { icon: Wallet,          label: "Whale exits" },
  { icon: Radio,           label: "Oracle divergence" },
  { icon: Repeat,          label: "Bridge outflows" },
  { icon: Gauge,           label: "Liquidity collapse" },
  { icon: Activity,        label: "Slippage spikes" },
];

const protectionActions = [
  { icon: SlidersHorizontal, label: "Adaptive fee control" },
  { icon: Timer,             label: "Withdrawal throttling" },
  { icon: Repeat,            label: "Liquidity rebalancing" },
  { icon: Zap,               label: "Emergency liquidity routing" },
  { icon: ShieldHalf,        label: "MEV suppression" },
];

export default function AegisLayers() {
  return (
    <section
      id="architecture"
      className="relative overflow-hidden bg-white py-24 md:py-28"
    >
      <div className="container-x relative z-10">
        <div className="mx-auto max-w-3xl text-center">
          <div className="pill mx-auto mb-4 inline-flex items-center gap-2">
            <CircuitBoard className="h-4 w-4" />
            Panic detection &amp; stabilization architecture
          </div>

          <h2 className="mt-6 text-4xl font-semibold leading-[1.05] tracking-tight text-skyglass-ink md:text-6xl">
            Detect stress.
            <br />
            <span className="text-skyglass-deep">Respond automatically.</span>
          </h2>

          <p className="mx-auto mt-6 max-w-2xl text-base leading-relaxed text-skyglass-muted md:text-lg">
            A two-layer protection system designed to detect stablecoin panic
            early and activate automated liquidity defense mechanisms before
            contagion spreads across DeFi markets.
          </p>
        </div>

        <div className="mt-20 grid gap-8 lg:grid-cols-[1fr_auto_1fr] lg:items-stretch">
          <div className="card flex flex-col !p-8">
            <div className="flex items-center gap-3">
              <div className="icon-chip h-11 w-11">
                <Activity className="h-5 w-5" />
              </div>

              <div>
                <div className="font-mono text-[10px] uppercase tracking-[0.22em] text-skyglass-deep">
                  Layer 01
                </div>
                <h3 className="mt-0.5 text-2xl font-semibold text-skyglass-ink">
                  Risk Oracle Engine
                </h3>
              </div>
            </div>

            <p className="mt-5 text-sm leading-relaxed text-skyglass-muted">
              Continuously monitors market, liquidity, and bridge activity to
              detect panic conditions in real time.
            </p>

            <div className="mt-8 flex-1 space-y-1">
              {monitoredSignals.map((signal) => {
                const Icon = signal.icon;
                return (
                  <div
                    key={signal.label}
                    className="flex items-center gap-3 border-b border-skyglass-deep/15 py-3.5"
                  >
                    <div className="icon-chip h-9 w-9">
                      <Icon className="h-4 w-4" />
                    </div>
                    <span className="text-sm text-skyglass-muted">
                      {signal.label}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>

          <div className="flex items-center justify-center">
            <div className="flex flex-col items-center gap-3">
              <div className="hidden h-24 w-px bg-gradient-to-b from-transparent via-skyglass-deep/50 to-transparent lg:block" />

              <div className="icon-chip flex h-14 w-14 items-center justify-center rounded-full !rounded-full">
                <ArrowRight className="h-6 w-6" />
              </div>

              <div className="hidden h-24 w-px bg-gradient-to-b from-transparent via-skyglass-deep/50 to-transparent lg:block" />
            </div>
          </div>

          <div className="card flex flex-col !p-8">
            <div className="flex items-center gap-3">
              <div className="icon-chip h-11 w-11">
                <ShieldHalf className="h-5 w-5" />
              </div>

              <div>
                <div className="font-mono text-[10px] uppercase tracking-[0.22em] text-skyglass-deep">
                  Layer 02
                </div>
                <h3 className="mt-0.5 text-2xl font-semibold text-skyglass-ink">
                  Circuit Breaker Layer
                </h3>
              </div>
            </div>

            <p className="mt-5 text-sm leading-relaxed text-skyglass-muted">
              Automatically activates protective controls the moment severe
              stress conditions are detected.
            </p>

            <div className="mt-8 flex-1 space-y-1">
              {protectionActions.map((action) => {
                const Icon = action.icon;
                return (
                  <div
                    key={action.label}
                    className="flex items-center gap-3 border-b border-skyglass-deep/15 py-3.5"
                  >
                    <div className="icon-chip h-9 w-9">
                      <Icon className="h-4 w-4" />
                    </div>
                    <span className="text-sm text-skyglass-muted">
                      {action.label}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
        </div>

        <div className="mt-16 border-t border-skyglass-deep/20 pt-8">
          <div className="flex flex-col gap-4 text-sm text-skyglass-muted md:flex-row md:items-center md:justify-between">
            <p>
              Designed for DEXs, lending markets, bridges, LPs, and stablecoin
              issuers.
            </p>

            <div className="flex items-center gap-2 font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-deep">
              <Zap className="h-3.5 w-3.5" />
              Real-time monitoring · automated protection
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
