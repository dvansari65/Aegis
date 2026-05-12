const items = [
  "USDC",
  "USDT",
  "PYUSD",
  "USDS",
  "FDUSD",
  "DAI",
  "USDe",
  "crvUSD",
  "GHO",
  "MIM",
  "FRAX",
  "TUSD",
];

export default function LogoMarquee() {
  return (
    <section
      data-testid="logo-marquee"
      className="border-y border-navy-border/40 bg-navy-mid/30 py-6"
    >
      <div className="container-x flex items-center gap-8">
        <div className="hidden shrink-0 font-mono text-[11px] uppercase tracking-[0.22em] text-cream-muted md:block">
          Monitoring
        </div>

        <div className="marquee-mask flex-1 overflow-hidden">
          <div className="flex w-max animate-ticker gap-12">
            {[...items, ...items].map((t, i) => (
              <span
                key={i}
                className="font-mono text-[15px] tracking-tight text-cream-dim"
              >
                <span className="mr-2 text-blue-500">●</span>
                {t}
              </span>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}