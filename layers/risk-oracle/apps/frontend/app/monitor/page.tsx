import type { Metadata } from "next";

import MonitorDashboard from "@/components/monitor-dashboard";

export const metadata: Metadata = {
  title: "Live monitor — Aegis Risk Oracle",
  description:
    "Stress score, liquidity health, depeg probability, circuit breaker mode, and secondary risk signals from the risk oracle API.",
};

export default function MonitorPage() {
  return <MonitorDashboard />;
}
