"use client";

import { Menu, X } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

const NAV_LINKS = [
  { label: "Architecture", href: "/#architecture" },
  { label: "Flow", href: "/#flow" },
  { label: "Integrations", href: "/#integrations" },
  { label: "Benefits", href: "/#benefits" },
] as const;

export default function Navbar() {
  const [open, setOpen] = useState(false);

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
          {NAV_LINKS.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className="text-sm text-skyglass-muted transition-colors hover:text-skyglass-ink"
            >
              {link.label}
            </Link>
          ))}
        </nav>

        <div className="hidden items-center gap-4 md:flex">
          <Link
            href="/monitor"
            className="text-sm font-medium text-skyglass-ink transition-colors hover:text-navy-700"
          >
            Monitor
          </Link>

          <Link
            href="/docs/protocol-overview"
            className="text-sm text-skyglass-muted transition-colors hover:text-skyglass-ink"
          >
            Docs
          </Link>

          <Link href="/#architecture" className="btn-primary !px-5 !py-2 !text-sm">
            Get Integrated
          </Link>
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
            <Link
              href="/monitor"
              onClick={() => setOpen(false)}
              className="py-3 font-medium text-skyglass-ink transition-colors hover:text-navy-700"
            >
              Monitor
            </Link>
            <Link
              href="/docs/protocol-overview"
              onClick={() => setOpen(false)}
              className="py-3 text-skyglass-muted transition-colors hover:text-skyglass-ink"
            >
              Docs
            </Link>
            {NAV_LINKS.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                onClick={() => setOpen(false)}
                className="py-3 text-skyglass-muted transition-colors hover:text-skyglass-ink"
              >
                {link.label}
              </Link>
            ))}
            <Link
              href="/#architecture"
              onClick={() => setOpen(false)}
              className="btn-primary mt-4 justify-center"
            >
              Get Integrated
            </Link>
          </div>
        </div>
      )}
    </header>
  );
}
