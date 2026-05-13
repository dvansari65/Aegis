export default function RiskOracleLayerPage() {
  return (
    <article className="mx-auto w-full max-w-4xl">
      <h1 className="text-3xl font-semibold tracking-tight text-skyglass-ink">
        Risk Oracle Layer
      </h1>
      <p className="mt-4 text-base leading-7 text-skyglass-muted">
        The Risk Oracle Layer is Layer 1 of Aegis. Its job is to detect stablecoin
        panic, compute risk, and publish a machine-readable risk feed that the
        Circuit Breaker Layer consumes.
      </p>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Components
      </h2>

      <ol className="mt-4 list-decimal space-y-3 pl-6 text-base leading-7 text-skyglass-muted">
        <li>
          <span className="font-semibold text-skyglass-ink">Data Ingestion Layer</span>
          <div className="mt-1">
            Collects raw Solana and oracle data from RPC, DEX pools, stablecoin
            accounts, price feeds, bridge flows, and whale activity.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Market State Builder</span>
          <div className="mt-1">
            Converts raw events into current stablecoin market state, such as price,
            liquidity depth, pool imbalance, slippage, volume, and flow direction.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Signal Engine</span>
          <div className="mt-1">
            Computes normalized risk signals from market state. Examples include peg
            deviation, oracle divergence, pool imbalance, whale exit velocity,
            liquidity depth drop, and bridge outflow velocity.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Stress Score Engine</span>
          <div className="mt-1">
            Combines risk signals into a single <code className="font-mono">0-100</code>{" "}
            stress score for each supported stablecoin.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Liquidity Health Engine</span>
          <div className="mt-1">
            Classifies liquidity condition as{" "}
            <code className="font-mono">Healthy</code>,{" "}
            <code className="font-mono">Watch</code>,{" "}
            <code className="font-mono">Stressed</code>,{" "}
            <code className="font-mono">Critical</code>, or{" "}
            <code className="font-mono">Severe</code>.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Depeg Probability Engine</span>
          <div className="mt-1">
            Estimates near-term peg instability risk using rule-based bands first,
            with room for historical and ML-based models later.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Panic Classification Engine</span>
          <div className="mt-1">
            Labels the type of panic event, such as liquidity panic, oracle
            dislocation, whale exit event, bridge outflow event, or systemic market
            stress.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Alert Engine</span>
          <div className="mt-1">
            Creates risk alerts for protocols, dashboards, SDK clients, and off-chain
            notification channels.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Oracle Publisher</span>
          <div className="mt-1">
            Takes the computed risk output and submits it to the on-chain Risk Oracle
            Program.
          </div>
        </li>
        <li>
          <span className="font-semibold text-skyglass-ink">Risk Oracle Program</span>
          <div className="mt-1">
            Stores the latest canonical risk state on Solana so integrated protocols
            can read it directly.
          </div>
        </li>
      </ol>

      <h2 className="mt-10 text-2xl font-semibold tracking-tight text-skyglass-ink">
        Production Flow
      </h2>
      <pre className="mt-4 overflow-x-auto rounded-xl border border-skyglass-deep/20 bg-navy-900 p-4 text-sm leading-7 text-skyglass-mist">
        <code>{`Data Ingestion
  -> Market State Builder
  -> Signal Engine
  -> Stress Score / Liquidity Health / Depeg Probability
  -> Panic Classification
  -> Oracle Publisher
  -> On-Chain Risk Oracle Program
  -> Circuit Breaker Layer / Protocol Integrations`}</code>
      </pre>
    </article>
  );
}

