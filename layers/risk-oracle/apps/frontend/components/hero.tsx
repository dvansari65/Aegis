import { Activity, ArrowUpRight, BookOpen, ShieldAlert, TrendingDown } from "lucide-react";

const SIGNALS = [
  { label: "Stress score", value: "91",      icon: Activity },
  { label: "Liquidity",   value: "Critical", icon: ShieldAlert },
  { label: "Depeg risk",  value: "High",     icon: TrendingDown },
];

export default function Hero() {
  return (
    <section
      id="top"
      data-testid="hero-section"
      className="relative overflow-hidden bg-white"
    >
      <div className="pointer-events-none absolute inset-0 bg-[linear-gradient(135deg,#FFFFFF_0%,#EAF6FF_45%,#C1E5FF_100%)]" />
      <div className="pointer-events-none absolute -right-28 top-20 h-[520px] w-[520px] rounded-[120px] bg-[#9CD5FF]/60 blur-3xl" />
      <div className="pointer-events-none absolute bottom-0 left-0 h-72 w-full bg-[linear-gradient(180deg,transparent,#FFFFFF)]" />

      <div className="container-x relative z-10 grid min-h-screen items-center gap-12 pb-20 pt-28 lg:grid-cols-[1.02fr_0.98fr] lg:pt-24">
        <div className="max-w-3xl">
          <div className="mb-7 animate-fade-up" style={{ animationDelay: "60ms" }}>
            <div data-testid="hero-badge" className="pill inline-flex">
              <span className="live-dot" />
              Real-time stablecoin panic control infrastructure
            </div>
          </div>

          <h1
            data-testid="hero-heading"
            className="animate-fade-up text-[46px] font-semibold leading-[0.98] tracking-tight text-skyglass-ink sm:text-6xl lg:text-[76px]"
            style={{ animationDelay: "120ms" }}
          >
            Stablecoin panic control for modern DeFi.
          </h1>

          <p
            data-testid="hero-subheading"
            className="animate-fade-up mt-7 max-w-xl text-base leading-relaxed text-skyglass-muted md:text-lg"
            style={{ animationDelay: "200ms" }}
          >
            A Solana-native risk oracle and circuit breaker protocol that
            computes stablecoin stress, publishes panic signals, and gives
            DeFi protocols the data they need to activate protective controls.
          </p>

          <div
            className="animate-fade-up mt-9 flex flex-col gap-3 sm:flex-row"
            style={{ animationDelay: "280ms" }}
          >
            <a
              href="#architecture"
              data-testid="hero-primary-cta"
              className="btn-primary"
            >
              View Architecture
              <ArrowUpRight className="h-4 w-4" strokeWidth={2.5} />
            </a>

            <a
              href="#docs"
              data-testid="hero-secondary-cta"
              className="btn-secondary"
            >
              <BookOpen className="h-4 w-4" />
              Read Documentation
            </a>
          </div>

          <div
            className="animate-fade-up mt-12 grid max-w-2xl gap-3 sm:grid-cols-3"
            style={{ animationDelay: "360ms" }}
          >
            {SIGNALS.map((signal) => {
              const Icon = signal.icon;
              return (
                <div
                  key={signal.label}
                  className="rounded-2xl border border-skyglass-deep/20 bg-white/70 p-5 shadow-[0_18px_50px_rgba(106,176,227,0.14)] backdrop-blur"
                >
                  <div className="flex items-center gap-2 text-skyglass-deep">
                    <Icon className="h-4 w-4" />
                    <span className="font-mono text-[10px] uppercase tracking-[0.18em] text-skyglass-muted">
                      {signal.label}
                    </span>
                  </div>

                  <p className="mt-4 text-3xl font-semibold text-skyglass-ink">
                    {signal.value}
                  </p>
                </div>
              );
            })}
          </div>
        </div>

        <div className="relative hidden min-h-[560px] lg:block">
          <div className="absolute right-0 top-0 h-[460px] w-[460px] rounded-[96px] bg-[#EAF6FF] shadow-[0_30px_90px_rgba(106,176,227,0.2)]" />
          <div className="absolute bottom-14 left-0 h-[330px] w-[380px] rounded-[72px] bg-[#C1E5FF]" />
          <div className="absolute bottom-0 right-10 h-[310px] w-[420px] rounded-[80px] bg-[#9CD5FF]" />
          <div className="absolute bottom-20 right-0 h-[220px] w-[520px] rounded-[56px] bg-[#6AB0E3] p-8 text-white shadow-[0_24px_70px_rgba(106,176,227,0.34)]">
            <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-white/76">
              Protection Mode
            </div>
            <div className="mt-6 text-5xl font-semibold tracking-tight">
              Armed
            </div>
            <div className="mt-8 grid grid-cols-3 gap-4 text-sm">
              <div>
                <div className="text-white/70">Fees</div>
                <div className="mt-1 font-semibold">Adaptive</div>
              </div>
              <div>
                <div className="text-white/70">Routes</div>
                <div className="mt-1 font-semibold">Guarded</div>
              </div>
              <div>
                <div className="text-white/70">Exits</div>
                <div className="mt-1 font-semibold">Queued</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
