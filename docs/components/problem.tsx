import { Droplets, Waves, ShieldOff } from "lucide-react";

const cards = [
  {
    icon: Droplets,
    title: "Liquidity Collapse",
    body: "During panic events, stablecoin liquidity pools can rapidly lose depth, causing severe slippage and unstable trading conditions.",
    span: "lg:col-span-2",
    accent: true,
  },
  {
    icon: Waves,
    title: "Panic-Driven Depegs",
    body: "Large withdrawals, whale exits, and aggressive arbitrage can push stablecoins below peg and accelerate market instability.",
    span: "lg:col-span-1",
  },
  {
    icon: ShieldOff,
    title: "No Coordinated Protection Layer",
    body: "Most DeFi protocols still lack real-time panic detection and automated stabilization infrastructure.",
    span: "lg:col-span-3",
  },
];

export default function Problem() {
  return (
    <section
      id="problem"
      data-testid="problem-section"
      className="relative bg-page py-24 md:py-32"
    >
      {/* Subtle background glow */}
      <div className="pointer-events-none absolute left-0 top-0 h-[400px] w-[600px] rounded-full bg-blue-900/30 blur-[120px]" />

      <div className="container-x relative z-10">
        <div className="max-w-3xl">
          <span className="pill" data-testid="problem-eyebrow">
            <span className="critical-dot live-dot" />
            The Problem
          </span>

          <h2 className="mt-6 text-3xl font-semibold leading-[1.05] tracking-tight text-cream md:text-5xl">
            Stablecoin panic spreads{" "}
            <span className="text-blue-500">faster</span>{" "}
            than DeFi infrastructure can react.
          </h2>
        </div>

        <div className="mt-14 grid gap-5 lg:grid-cols-3">
          {cards.map((c, i) => (
            <article
              key={c.title}
              data-testid={`problem-card-${i + 1}`}
              className={`card group relative overflow-hidden ${c.span} ${
                c.accent
                  ? "bg-gradient-to-br from-navy-mid to-navy-deep lg:row-span-1"
                  : ""
              }`}
            >
              {c.accent && (
                <div className="pointer-events-none absolute -right-10 -top-10 h-48 w-48 rounded-full bg-blue-700/15 blur-3xl" />
              )}

              <div className="relative">
                <span className="icon-chip grid h-11 w-11 place-items-center">
                  <c.icon className="h-5 w-5" strokeWidth={2} />
                </span>

                <h3 className="mt-6 text-xl font-semibold tracking-tight text-cream md:text-2xl">
                  {c.title}
                </h3>

                <p className="mt-3 max-w-md text-[15px] leading-relaxed text-cream-dim">
                  {c.body}
                </p>

                <div className="mt-6 font-mono text-[11px] uppercase tracking-[0.2em] text-cream-muted">
                  0{i + 1} — Risk Vector
                </div>
              </div>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}