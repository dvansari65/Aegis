import {
  Repeat,
  Landmark,
  Coins,
  Droplets,
  Network,
  ArrowUpRight,
} from "lucide-react";

const types = [
  { icon: Repeat,    label: "DEXs" },
  { icon: Landmark,  label: "Lending Protocols" },
  { icon: Coins,     label: "Stablecoin Protocols" },
  { icon: Droplets,  label: "Liquidity Pools" },
  { icon: Network,   label: "Cross-Chain Bridges" },
];

const examples = [
  {
    tag: "Example A",
    title: "DEX Protection",
    body: "DEXs integrate our stress feed and adaptive fee infrastructure to reduce liquidity collapse during panic events.",
    highlight: "swap_fee += stress_multiplier()",
    code: [
      "import { aegis } from '@aegis/sdk'",
      "",
      "const score = await aegis.stressScore('USDC')",
      "if (score > 85) {",
      "  pool.setSwapFee(baseFee + 0.4)",
      "  pool.routeReserveLiquidity()",
      "}",
    ],
  },
  {
    tag: "Example B",
    title: "Lending Market Protection",
    body: "Lending protocols use our stress oracle to tighten collateral requirements and reduce liquidation cascades during stablecoin instability.",
    highlight: "ltv -= depeg_probability()",
    code: [
      "const depeg = await aegis.depegProbability('USDT')",
      "if (depeg === 'HIGH') {",
      "  market.setMaxLTV(0.72)",
      "  market.pauseLiquidationCascade()",
      "}",
    ],
  },
];

export default function Integrations() {
  return (
    <section
      id="integrations"
      data-testid="integrations-section"
      className="relative border-t border-navy-border/40 bg-section py-24 md:py-32"
    >
      <div className="container-x">

        {/* Header */}
        <div className="flex max-w-5xl flex-col justify-between gap-6 md:flex-row md:items-end">
          <div>
            <span className="pill">Integrations</span>

            <h2 className="mt-6 text-3xl font-semibold leading-[1.05] tracking-tight text-cream md:text-5xl">
              Designed for{" "}
              <span className="text-blue-300">DeFi infrastructure</span>.
            </h2>

            <p className="mt-5 max-w-2xl leading-relaxed text-cream-dim">
              Protocols integrate our risk oracle feeds and circuit breaker
              modules to automatically respond to stablecoin stress events.
            </p>
          </div>
        </div>

        {/* Type chips */}
        <div className="mt-12 grid gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5">
          {types.map((t) => (
            <div
              key={t.label}
              data-testid={`integration-type-${t.label
                .toLowerCase()
                .replace(/\s+/g, "-")}`}
              className="group rounded-2xl border border-navy-border bg-navy-mid px-5 py-5 transition-all duration-300 hover:-translate-y-1 hover:border-blue-700/60 hover:shadow-[0_8px_32px_rgba(0,78,224,0.2)]"
            >
              <div className="flex items-center justify-between">
                <span className="icon-chip grid h-10 w-10 place-items-center rounded-lg">
                  <t.icon className="h-4 w-4" />
                </span>

                <ArrowUpRight className="h-4 w-4 text-cream-muted transition-colors group-hover:text-blue-300" />
              </div>

              <div className="mt-5 text-[15px] font-medium tracking-tight text-cream">
                {t.label}
              </div>
            </div>
          ))}
        </div>

        {/* Code examples */}
        <div className="mt-12 grid gap-6 lg:grid-cols-2">
          {examples.map((e) => (
            <div
              key={e.title}
              data-testid={`integration-example-${e.title
                .toLowerCase()
                .replace(/\s+/g, "-")}`}
              className="card flex flex-col overflow-hidden !p-0"
            >
              {/* Header */}
              <div className="border-b border-navy-border px-7 pb-5 pt-7">
                <div className="flex items-center justify-between">
                  <span className="font-mono text-[10px] uppercase tracking-[0.22em] text-blue-500">
                    {e.tag}
                  </span>

                  <span className="font-mono text-[10px] uppercase tracking-[0.2em] text-cream-muted">
                    Live Endpoint
                  </span>
                </div>

                <h3 className="mt-4 text-xl font-semibold tracking-tight text-cream md:text-2xl">
                  {e.title}
                </h3>

                <p className="mt-2 text-sm leading-relaxed text-cream-dim">
                  {e.body}
                </p>
              </div>

              {/* Code block */}
              <div className="code-block flex-1">
                <div className="mb-3 text-blue-300">
                  {"// "}
                  {e.highlight}
                </div>

                {e.code.map((l, i) => (
                  <div key={i} className="flex gap-4">
                    <span className="w-4 select-none text-right text-cream-muted">
                      {i + 1}
                    </span>
                    <span className="whitespace-pre text-cream-dim">
                      {l || "\u00A0"}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
