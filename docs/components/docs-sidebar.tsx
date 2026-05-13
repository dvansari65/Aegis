"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

export type DocsNavItem = {
  title: string;
  href: string;
};

function isActive(pathname: string, href: string) {
  return pathname === href || pathname.startsWith(`${href}/`);
}

export default function DocsSidebar({ items }: { items: DocsNavItem[] }) {
  const pathname = usePathname();

  return (
    <nav aria-label="Documentation" className="flex flex-col gap-1.5">
      {items.map((item) => {
        const active = isActive(pathname, item.href);
        return (
          <Link
            key={item.href}
            href={item.href}
            className={[
              "rounded-lg px-3 py-2 text-sm transition-colors",
              active
                ? "bg-skyglass-ice text-skyglass-ink"
                : "text-skyglass-muted hover:bg-skyglass-mist hover:text-skyglass-ink",
            ].join(" ")}
          >
            {item.title}
          </Link>
        );
      })}
    </nav>
  );
}

