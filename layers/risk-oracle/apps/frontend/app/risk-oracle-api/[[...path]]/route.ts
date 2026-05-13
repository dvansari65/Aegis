import type { NextRequest } from "next/server";

export const dynamic = "force-dynamic";

const DEFAULT_UPSTREAM = "http://127.0.0.1:8080";

/**
 * Server-side proxy: browser calls same-origin `/risk-oracle-api/v1/risk/USDC`,
 * this route forwards to `risk-oracle-api` (see `RISK_ORACLE_API_URL`).
 */
export async function GET(
  _req: NextRequest,
  ctx: { params: Promise<{ path?: string[] }> },
) {
  const { path } = await ctx.params;
  const parts = path ?? [];
  const upstreamPath = parts.join("/");
  const base = (process.env.RISK_ORACLE_API_URL ?? DEFAULT_UPSTREAM).replace(/\/$/, "");
  const url = `${base}/${upstreamPath}`;

  let res: Response;
  try {
    res = await fetch(url, {
      cache: "no-store",
      headers: { Accept: "application/json" },
      signal: AbortSignal.timeout(15_000),
    });
  } catch (e) {
    return Response.json(
      {
        error: "upstream_unreachable",
        detail: String(e),
        hint: "Start risk-oracle-api (cargo run in layers/risk-oracle/apps/api) or set RISK_ORACLE_API_URL",
      },
      { status: 502, headers: { "Cache-Control": "no-store" } },
    );
  }

  const body = await res.text();
  return new Response(body, {
    status: res.status,
    headers: {
      "Content-Type": res.headers.get("content-type") ?? "application/json",
      "Cache-Control": "no-store, must-revalidate",
    },
  });
}
