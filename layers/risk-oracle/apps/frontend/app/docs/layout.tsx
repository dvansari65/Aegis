import Navbar from "@/components/navbar";
import DocsSidebar from "@/components/docs-sidebar";

const DOCS_NAV = [
  { title: "Live monitor", href: "/monitor" },
  { title: "Protocol Overview", href: "/docs/protocol-overview" },
  { title: "Risk Oracle Layer", href: "/docs/risk-oracle-layer" },
  { title: "Integrations", href: "/docs/integrations" },
] as const;

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-white">
      <Navbar />
      <div className="mx-auto w-full max-w-7xl px-5 pb-16 pt-20 md:px-8">
        <aside className="fixed top-20 hidden h-[calc(100vh-5rem)] w-72 overflow-y-auto rounded-2xl border border-skyglass-deep/20 bg-white p-6 md:block">
          <div className="mb-4 font-mono text-[11px] uppercase tracking-[0.22em] text-skyglass-muted">
            Documentation
          </div>
          <DocsSidebar items={[...DOCS_NAV]} />
        </aside>

        <main className="min-w-0 md:pl-72">
          <div className="rounded-2xl border border-skyglass-deep/20 bg-white p-6 sm:p-8 lg:p-12">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
}
