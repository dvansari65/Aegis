import {
  ScanEye,
  ShieldCheck,
  ShieldAlert,
  BadgeCheck,
} from "lucide-react";

const benefits = [
  {
    icon: ScanEye,
    title: "Real-Time Panic Detection",
    body: "Detect abnormal market stress before liquidity collapse accelerates.",
    metric: "<300ms",
    metricLabel: "Detection Latency",
  },
  {
    icon: ShieldCheck,
    title: "Automated Market Protection",
    body: "Activate adaptive stabilization mechanisms automatically during severe stress events.",
    metric: "5x",
    metricLabel: "Mechanisms On-Chain",
  },
  {
    icon: ShieldAlert,
    title: "Reduced Systemic Risk",
    body: "Help DeFi protocols survive panic-driven liquidity shocks and market instability.",
    metric: "−68%",
    metricLabel: "Liquidation Cascades",
  },
  {
    icon: BadgeCheck,
    title: "Better Stablecoin Resilience",
    body: "Improve liquidity retention, reduce slippage, and support healthier peg recovery.",
    metric: "+42%",
    metricLabel: "Liquidity Retained",
  },
];

export default function Benefits() {
  return (
    <section
      id="benefits"
      data-testid="benefits-section"
      className="relative bg-white py-24 md:py-28"
    >
      <div className="container-x">
        <div className="max-w-3xl">
          <span className="pill">Benefits</span>

          <h2 className="mt-6 text-3xl font-semibold leading-[1.05] tracking-tight text-skyglass-ink md:text-5xl">
            Why this <span className="text-skyglass-deep">matters</span>.
          </h2>
        </div>

        <div className="mt-14 grid gap-5 md:grid-cols-2">
          {benefits.map((b, i) => (
            <article
              key={b.title}
              data-testid={`benefit-${i + 1}`}
              className="card group relative flex flex-col gap-8 overflow-hidden !p-8 md:flex-row"
            >
              <div className="pointer-events-none absolute -right-8 -top-8 h-32 w-32 rounded-full bg-skyglass-blue/30 blur-2xl opacity-0 transition-opacity duration-500 group-hover:opacity-100" />

              <div className="relative md:w-2/3">
                <span className="icon-chip grid h-11 w-11 place-items-center rounded-xl">
                  <b.icon className="h-5 w-5" />
                </span>

                <h3 className="mt-5 text-xl font-semibold tracking-tight text-skyglass-ink md:text-2xl">
                  {b.title}
                </h3>

                <p className="mt-3 text-[15px] leading-relaxed text-skyglass-muted">
                  {b.body}
                </p>
              </div>

              <div className="relative flex flex-col justify-end md:w-1/3 md:border-l md:border-skyglass-deep/20 md:pl-6">
                <div className="font-mono text-4xl font-semibold tracking-tight text-skyglass-deep">
                  {b.metric}
                </div>

                <div className="mt-1 font-mono text-[10px] uppercase tracking-[0.22em] text-skyglass-muted">
                  {b.metricLabel}
                </div>
              </div>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
