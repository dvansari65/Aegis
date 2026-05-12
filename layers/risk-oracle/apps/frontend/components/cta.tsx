import { ArrowUpRight, Mail } from "lucide-react";

export default function FinalCTA() {
  return (
    <section
      id="cta"
      data-testid="final-cta-section"
      className="relative bg-section py-24 md:py-28"
    >
      <div className="container-x">
        <div className="relative overflow-hidden rounded-3xl border border-navy-border bg-gradient-to-br from-navy-mid via-navy-deep to-navy-deepest p-10 md:p-16">

          {/* Decorative glows */}
          <div className="pointer-events-none absolute -right-24 -top-24 h-[420px] w-[420px] rounded-full bg-blue-700/20 blur-3xl" />
          <div className="pointer-events-none absolute bottom-0 left-0 h-[300px] w-[300px] rounded-full bg-blue-900/30 blur-3xl" />
          <div className="grid-bg pointer-events-none absolute inset-0 opacity-[0.08] [mask-image:radial-gradient(ellipse_at_right,black_30%,transparent_70%)]" />

          <div className="relative grid items-center gap-10 lg:grid-cols-12">

            {/* Left — CTA text */}
            <div className="lg:col-span-7">
              <span className="pill">
                <span className="live-dot" />
                Onboarding Partners
              </span>

              <h2 className="mt-6 max-w-2xl text-3xl font-semibold leading-[1.05] tracking-tight text-cream md:text-5xl">
                Stress-test your protocol before the{" "}
                <span
                  style={{
                    background: "linear-gradient(135deg, #1883FF 0%, #99CAFF 100%)",
                    WebkitBackgroundClip: "text",
                    WebkitTextFillColor: "transparent",
                    backgroundClip: "text",
                  }}
                >
                  market does it for you.
                </span>
              </h2>

              <p className="mt-5 max-w-xl leading-relaxed text-cream-dim">
                Integrate Aegis risk feeds and circuit breaker modules in days.
                Built for production DeFi on Solana — battle-tested
                infrastructure, zero compromises.
              </p>

              <div className="mt-8 flex flex-col gap-3 sm:flex-row">
                <a
                  href="#"
                  data-testid="final-cta-primary"
                  className="btn-primary"
                >
                  Get Integrated
                  <ArrowUpRight className="h-4 w-4" />
                </a>

                <a
                  href="#"
                  data-testid="final-cta-secondary"
                  className="btn-secondary"
                >
                  <Mail className="h-4 w-4" />
                  Talk to the Team
                </a>
              </div>
            </div>

            {/* Right — Status panel */}
            <div className="lg:col-span-5">
              <div className="rounded-2xl border border-navy-border bg-navy-deepest/80 p-6 backdrop-blur-sm">
                <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-cream-muted">
                  System Status
                </div>

                <div className="mt-4 space-y-3">
                  {[
                    { label: "Risk Oracle",       status: "Operational", ok: true },
                    { label: "Circuit Breaker",   status: "Armed",       ok: false },
                    { label: "Bridge Monitoring", status: "Operational", ok: true },
                    { label: "Pyth Feeds",        status: "Operational", ok: true },
                  ].map((row) => (
                    <div
                      key={row.label}
                      className="flex items-center justify-between text-sm"
                    >
                      <span className="text-cream-dim">{row.label}</span>

                      <span
                        className={`font-mono text-[12px] uppercase tracking-[0.18em] ${
                          row.ok ? "text-status-ok" : "text-blue-300"
                        }`}
                      >
                        ● {row.status}
                      </span>
                    </div>
                  ))}
                </div>

                <div className="mt-6 flex items-center justify-between border-t border-navy-border pt-5 font-mono text-[11px] uppercase tracking-[0.2em] text-cream-muted">
                  <span>Uptime · 99.99%</span>
                  <span>Region · Global</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}