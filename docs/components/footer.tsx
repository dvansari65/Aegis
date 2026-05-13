import Image from "next/image";
import GitHub from "./github-icon";

const cols = [
  {
    title: "Protocol",
    links: [
      { label: "Architecture", href: "/#architecture" },
      { label: "Flow", href: "/#flow" },
    ],
  },
  {
    title: "Developers",
    links: [
      { label: "Docs", href: "/docs/protocol-overview" },
      { label: "Integrations", href: "/docs/integrations" },
    ],
  },
  {
    title: "Links",
    links: [
      { label: "GitHub", href: "https://github.com/dvansari65/stable-coin-shock-absorber" },
    ],
  },
];

export default function Footer() {
  return (
    <footer
      data-testid="footer"
      className="relative border-t border-skyglass-deep/20 bg-[#EAF6FF] pb-10 pt-16"
    >
      <div className="container-x">
        <div className="grid gap-10 lg:grid-cols-12">
          <div className="lg:col-span-5">
            <div className="flex items-center">
              <span className="flex h-20 w-60 items-center justify-start">
                <Image
                  src="/aegis-logo.svg"
                  alt="Aegis"
                  width={240}
                  height={184}
                  className="h-20 w-auto object-contain"
                />
              </span>
            </div>

            <p className="mt-5 max-w-md leading-relaxed text-skyglass-muted">
              Real-time risk oracle and circuit breaker infrastructure for the
              Solana DeFi ecosystem.
            </p>

            <div className="mt-6 flex items-center gap-2.5">
              {[
                { icon: GitHub, label: "GitHub", href: "https://github.com/dvansari65/stable-coin-shock-absorber" },
              ].map((s) => (
                <a
                  key={s.label}
                  href={s.href}
                  data-testid={`social-${s.label.toLowerCase()}`}
                  aria-label={s.label}
                  rel="noreferrer"
                  target="_blank"
                  className="grid h-10 w-10 place-items-center rounded-lg border border-skyglass-deep/20 bg-white/60 text-skyglass-muted transition-all hover:border-skyglass-deep/50 hover:bg-white hover:text-skyglass-deep"
                >
                  <s.icon className="h-4 w-4" />
                </a>
              ))}
            </div>
          </div>

          <div className="grid gap-8 sm:grid-cols-3 lg:col-span-7">
            {cols.map((col) => (
              <div key={col.title}>
                <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
                  {col.title}
                </div>

                <ul className="mt-4 space-y-2.5">
                  {col.links.map((l) => (
                    <li key={l.label}>
                      <a
                        href={l.href}
                        rel={l.href.startsWith("http") ? "noreferrer" : undefined}
                        target={l.href.startsWith("http") ? "_blank" : undefined}
                        className="text-sm text-skyglass-muted transition-colors hover:text-skyglass-ink"
                      >
                        {l.label}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>

        <div className="mt-16 flex flex-col items-start justify-between gap-4 border-t border-skyglass-deep/20 pt-6 md:flex-row md:items-center">
          <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
            © {new Date().getFullYear()} Aegis Labs · All systems nominal
          </div>

          <div className="flex items-center gap-5 text-[12px] text-skyglass-muted">
            <span className="font-mono">
              Built on{" "}
              <span className="text-skyglass-deep">Solana</span>
            </span>
          </div>
        </div>
      </div>
    </footer>
  );
}
