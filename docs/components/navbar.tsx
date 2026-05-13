"use client";

import { Menu, X } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useMemo, useState } from "react";

const LANDING_NAV_LINKS = [
  { label: "Architecture", id: "architecture" },
  { label: "Flow", id: "flow" },
  { label: "Benefits", id: "benefits" },
] as const;

const DOCS_NAV_LINKS = [
  { label: "Home", href: "/" },
  { label: "Overview", href: "/docs/protocol-overview" },
  { label: "Risk Oracle", href: "/docs/risk-oracle-layer" },
  { label: "Integrations", href: "/docs/integrations" },
] as const;

function isActiveDoc(pathname: string, href: string) {
  return pathname === href || pathname.startsWith(`${href}/`);
}

export default function Navbar() {
  const [open, setOpen] = useState(false);
  const pathname = usePathname();
  const onDocs = pathname.startsWith("/docs");
  const onLanding = pathname === "/";
  const [activeSection, setActiveSection] = useState<string | null>(null);

  const landingLinks = useMemo(() => {
    return LANDING_NAV_LINKS.map((l) => ({
      label: l.label,
      id: l.id,
      href: onLanding ? `#${l.id}` : `/#${l.id}`,
    }));
  }, [onLanding]);

  useEffect(() => {
    if (!onLanding) return;

    const ids = LANDING_NAV_LINKS.map((l) => l.id);
    const nodes = ids
      .map((id) => document.getElementById(id))
      .filter((n): n is HTMLElement => Boolean(n));
    if (!nodes.length) return;

    const obs = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((e) => e.isIntersecting)
          .sort((a, b) => (b.intersectionRatio ?? 0) - (a.intersectionRatio ?? 0))[0];
        if (visible?.target?.id) setActiveSection(visible.target.id);
      },
      {
        root: null,
        threshold: [0.2, 0.35, 0.5, 0.65],
        rootMargin: "-20% 0px -65% 0px",
      }
    );

    nodes.forEach((n) => obs.observe(n));
    return () => obs.disconnect();
  }, [onLanding]);

  return (
    <header className="fixed inset-x-0 top-0 z-50 glass">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-5 md:px-8">
        <Link href="/" className="flex items-center">
          <div className="flex h-16 w-56 items-center justify-start">
            <Image
              src="/aegis-logo.svg"
              alt="Aegis"
              width={224}
              height={172}
              className="h-16 w-auto object-contain"
              priority
            />
          </div>
        </Link>

        <nav className="hidden items-center gap-8 md:flex">
          {onDocs ? (
            <>
              {DOCS_NAV_LINKS.map((link) => (
                <Link
                  key={link.href}
                  href={link.href}
                  className={[
                    "text-sm transition-colors hover:text-skyglass-ink",
                    isActiveDoc(pathname, link.href)
                      ? "text-skyglass-ink"
                      : "text-skyglass-muted",
                  ].join(" ")}
                >
                  {link.label}
                </Link>
              ))}
            </>
          ) : (
            landingLinks.map((link) => (
              <Link
                key={link.id}
                href={link.href}
                className={[
                  "text-sm transition-colors hover:text-skyglass-ink",
                  onLanding && activeSection === link.id
                    ? "text-skyglass-ink"
                    : "text-skyglass-muted",
                ].join(" ")}
              >
                {link.label}
              </Link>
            ))
          )}
        </nav>

        <div className="hidden items-center gap-4 md:flex">
          {onDocs ? (
            <Link href="/" className="btn-secondary px-5! py-2! text-sm!">
              Back to landing
            </Link>
          ) : (
            <>
              <Link
                href="/docs/protocol-overview"
                className="text-sm text-skyglass-muted transition-colors hover:text-skyglass-ink"
              >
                Docs
              </Link>
              <Link
                href={onLanding ? "#architecture" : "/#architecture"}
                className="btn-primary px-5! py-2! text-sm!"
              >
                View Architecture
              </Link>
            </>
          )}
        </div>

        <button
          onClick={() => setOpen(!open)}
          className="text-skyglass-ink md:hidden"
          aria-label="Toggle menu"
        >
          {open ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
        </button>
      </div>

      {open && (
        <div className="border-t border-skyglass-deep/20 bg-white md:hidden">
          <div className="flex flex-col px-6 py-4">
            {onDocs ? (
              <>
                {DOCS_NAV_LINKS.map((link) => (
                  <Link
                    key={link.href}
                    href={link.href}
                    onClick={() => setOpen(false)}
                    className={[
                      "py-3 transition-colors hover:text-skyglass-ink",
                      isActiveDoc(pathname, link.href)
                        ? "text-skyglass-ink"
                        : "text-skyglass-muted",
                    ].join(" ")}
                  >
                    {link.label}
                  </Link>
                ))}
                <Link
                  href="/"
                  onClick={() => setOpen(false)}
                  className="btn-primary mt-4 justify-center"
                >
                  Back to landing
                </Link>
              </>
            ) : (
              <>
                {landingLinks.map((link) => (
                  <Link
                    key={link.id}
                    href={link.href}
                    onClick={() => setOpen(false)}
                    className={[
                      "py-3 transition-colors hover:text-skyglass-ink",
                      onLanding && activeSection === link.id
                        ? "text-skyglass-ink"
                        : "text-skyglass-muted",
                    ].join(" ")}
                  >
                    {link.label}
                  </Link>
                ))}
                <Link
                  href="/docs/protocol-overview"
                  onClick={() => setOpen(false)}
                  className="btn-primary mt-4 justify-center"
                >
                  Read Docs
                </Link>
              </>
            )}
          </div>
        </div>
      )}
    </header>
  );
}
