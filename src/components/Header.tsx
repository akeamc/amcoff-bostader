import Link from "next/link";
import { PropsWithChildren } from "react";

function Entry({ children, href }: PropsWithChildren<{ href: string }>) {
  return (
    <Link
      className="px-3 py-2 text-sm font-medium transition-colors duration-100 hover:text-green-600"
      href={href}
    >
      {children}
    </Link>
  );
}

export default function Header() {
  return (
    <header className="flex items-center p-4">
      <div className="flex flex-1 justify-center">
        <span className="mr-auto font-serif text-xl font-medium">
          A
          <Link
            className="text-neutral-400 hover:underline"
            href="https://amcoff.net"
          >
            (mcof)
          </Link>
          F <span className="max-sm:hidden">Bostäder</span>
        </span>
      </div>
      <div className="flex flex-1 justify-center">
        <nav className="flex items-center rounded-[9999px] border px-3 shadow-sm">
          <Entry href="/">Start</Entry>
          <Entry href="/history">History</Entry>
        </nav>
      </div>
      <div className="flex flex-1 justify-center">
        <button className="ml-auto">Log in</button>
      </div>
    </header>
  );
}
