"use client";

import { useVacancies } from "@/lib/hooks";
import classNames from "classnames";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { PropsWithChildren } from "react";

export function AvailableLink() {
  const { data } = useVacancies();
  const count = data?.length;

  return (
    <NavLink href="/" className="inline-flex items-center">
      Lediga bostäder
      {typeof count === "number" && (
        <span className="ml-1 rounded-[9999px] bg-green-100 px-2 py-0.5 text-xs text-green-600">
          {count}
        </span>
      )}
    </NavLink>
  );
}

export function NavLink({
  children,
  href,
  className,
}: PropsWithChildren<{ href: string; className?: string }>) {
  const active = usePathname() === href;

  return (
    <Link
      className={classNames(
        "px-3 py-2 text-sm font-medium transition-colors duration-100 hover:text-green-600",
        { "text-green-600": active },
        className,
      )}
      href={href}
    >
      {children}
    </Link>
  );
}
