export default function ProtocolOverviewPage() {
  return (
    <article className="mx-auto w-full max-w-4xl">
      <h1 className="text-3xl font-semibold tracking-tight text-skyglass-ink">
        Protocol Overview
      </h1>
      <p className="mt-4 text-base leading-7 text-skyglass-muted">
        Aegis is designed as two separate but connected layers.
      </p>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Layer 1: Risk Oracle
      </h2>
      <p className="mt-3 text-base leading-7 text-skyglass-muted">
        Layer 1 observes the market and produces trusted risk state.
      </p>

      <div className="mt-5 rounded-xl border border-skyglass-deep/20 bg-skyglass-mist p-5">
        <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
          Responsibilities
        </div>
        <ul className="mt-3 list-disc space-y-1.5 pl-5 text-sm leading-6 text-skyglass-muted">
          <li>collect stablecoin market data</li>
          <li>build current market state</li>
          <li>compute risk signals</li>
          <li>score panic severity</li>
          <li>estimate depeg probability</li>
          <li>publish risk feeds on-chain</li>
          <li>notify off-chain consumers</li>
        </ul>
      </div>

      <p className="mt-5 text-base leading-7 text-skyglass-muted">
        Layer 1 does not control liquidity or user actions. It only detects and
        publishes risk.
      </p>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Layer 2: Circuit Breaker
      </h2>
      <p className="mt-3 text-base leading-7 text-skyglass-muted">
        Layer 2 consumes risk oracle output and turns it into defensive actions.
      </p>

      <div className="mt-5 rounded-xl border border-skyglass-deep/20 bg-skyglass-mist p-5">
        <div className="font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
          Responsibilities
        </div>
        <ul className="mt-3 list-disc space-y-1.5 pl-5 text-sm leading-6 text-skyglass-muted">
          <li>activate adaptive fees</li>
          <li>throttle risky withdrawals</li>
          <li>coordinate liquidity routing</li>
          <li>suppress toxic arbitrage patterns</li>
          <li>trigger emergency liquidity policies</li>
          <li>
            expose integration modules for DEXs, lending markets, bridges, and
            stablecoin pools
          </li>
        </ul>
      </div>

      <p className="mt-5 text-base leading-7 text-skyglass-muted">
        Layer 2 should depend on Layer 1 outputs, but Layer 1 should not depend
        on Layer 2.
      </p>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Who Uses Aegis
      </h2>
      <p className="mt-3 text-base leading-7 text-skyglass-muted">
        See{" "}
        <a
          href="/docs/integrations"
          className="font-semibold text-navy-700 hover:text-navy-600"
        >
          Integrations
        </a>{" "}
        for the concrete platform categories that can use Aegis (DEXs, lending
        protocols, stablecoin issuers, liquidity infrastructure, and bridges)
        and what each category can do with the risk feed and circuit breaker
        layer.
      </p>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Data Flow
      </h2>
      <pre className="mt-4 overflow-x-auto rounded-xl border border-skyglass-deep/20 bg-navy-900 p-4 text-sm leading-7 text-skyglass-mist">
        <code>{`Solana / Oracle / DEX / Bridge Data
  -> Layer 1 Risk Oracle
  -> On-Chain Risk Feed
  -> Layer 2 Circuit Breaker
  -> Integrated Protocol Actions`}</code>
      </pre>
    </article>
  );
}
