"use client";

import { useCallback, useEffect, useState } from "react";

import { fetchMonitor, type MonitorResponse } from "@/lib/risk-oracle";

type State =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "ok"; data: MonitorResponse }
  | { status: "error"; message: string };

/** Fast enough to feel “live” on the demo stream; Solana mode uses the same cadence. */
const POLL_MS = 2_000;

export function useMonitor(symbol: string) {
  const [state, setState] = useState<State>({ status: "loading" });
  const [isRefreshing, setIsRefreshing] = useState(false);

  const load = useCallback(
    async (signal: AbortSignal, soft: boolean) => {
      if (soft) setIsRefreshing(true);
      else setState({ status: "loading" });
      try {
        const data = await fetchMonitor(symbol, signal);
        if (signal.aborted) return;
        setState({ status: "ok", data });
      } catch (e) {
        if (signal.aborted) return;
        const message = e instanceof Error ? e.message : "Request failed";
        setState({ status: "error", message });
      } finally {
        if (!signal.aborted) setIsRefreshing(false);
      }
    },
    [symbol],
  );

  useEffect(() => {
    const ac = new AbortController();
    void load(ac.signal, false);
    const id = window.setInterval(() => {
      void load(ac.signal, true);
    }, POLL_MS);
    return () => {
      ac.abort();
      window.clearInterval(id);
    };
  }, [load]);

  return { state, isRefreshing };
}
