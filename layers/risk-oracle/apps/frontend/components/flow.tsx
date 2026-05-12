import { Eye, AlertTriangle, ShieldCheck, Activity } from "lucide-react";

const steps = [
  {
    n: "01",
    icon: Eye,
    title: "Monitor Markets",
    body: "The protocol continuously monitors stablecoin pools, swaps, whale wallets, bridge activity, and oracle feeds on Solana.",
  },
  {
    n: "02",
    icon: AlertTriangle,
    title: "Detect Panic",
    body: "The risk engine computes stress scores and identifies abnormal market behavior before panic accelerates.",
  },
  {
    n: "03",
    icon: ShieldCheck,
    title: "Activate Protections",
    body: "When stress exceeds critical thresholds, the circuit breaker layer activates adaptive market protections automatically.",
  },
  {
    n: "04",
    icon: Activity,
    title: "Stabilize Markets",
    body: "The protocol helps reduce panic-driven liquidity collapse and improves market stability during stress events.",
  },
];

export default function Flow() {
  return (
    <section
      id="flow"
      data-testid="flow-section"
      className="relative bg-[#EAF6FF] py-24 md:py-28"
    >
      <div className="container-x">
        <div className="max-w-3xl">
          <span className="pill">End-to-End Flow</span>

          <h2 className="mt-6 text-3xl font-semibold leading-[1.05] tracking-tight text-skyglass-ink md:text-5xl">
            How the protocol works in{" "}
            <span className="text-skyglass-deep">production</span>.
          </h2>
        </div>

        <div className="relative mt-16">
          <div className="absolute left-12 right-12 top-12 hidden h-px bg-gradient-to-r from-transparent via-skyglass-deep/45 to-transparent lg:block" />

          <ol className="grid gap-6 lg:grid-cols-4">
            {steps.map((s, i) => (
              <li
                key={s.n}
                data-testid={`flow-step-${i + 1}`}
                className="relative"
              >
                <div className="card h-full !p-7">
                  <div className="flex items-center justify-between">
                    <span className="icon-chip relative z-10 grid h-12 w-12 place-items-center rounded-xl">
                      <s.icon className="h-5 w-5" />
                    </span>

                    <span className="font-mono text-2xl font-semibold tracking-tight text-skyglass-blue">
                      {s.n}
                    </span>
                  </div>

                  <h3 className="mt-6 text-lg font-semibold tracking-tight text-skyglass-ink">
                    {s.title}
                  </h3>

                  <p className="mt-2 text-sm leading-relaxed text-skyglass-muted">
                    {s.body}
                  </p>
                </div>
              </li>
            ))}
          </ol>
        </div>
      </div>
    </section>
  );
}
