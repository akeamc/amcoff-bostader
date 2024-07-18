import Link from "next/link";
import { PropsWithChildren, ReactNode } from "react";

function FooterLink({ href, children }: PropsWithChildren<{ href: string }>) {
  return (
    <li className="text-sm">
      <Link className="hover:text-white" href={href}>
        {children}
      </Link>
    </li>
  );
}

function LinkGroup({
  header,
  children,
}: PropsWithChildren<{ header: ReactNode }>) {
  return (
    <section>
      <h3 className="text-xs font-medium uppercase tracking-wide text-white">
        {header}
      </h3>
      <ul className="mt-2 space-y-1">{children}</ul>
    </section>
  );
}

export default function Footer() {
  return (
    <div className="mt-auto pt-8">
      <footer className="w-full bg-green-950 p-8 text-neutral-400">
        <div className="mx-auto grid max-w-screen-lg gap-x-4 gap-y-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5">
          <span className="mr-auto font-serif text-xl font-medium text-white">
            A
            <Link
              className="text-neutral-400 hover:underline"
              href="https://amcoff.net"
            >
              (mcof)
            </Link>
            F Bostäder
          </span>
          <LinkGroup header="Vacancies">
            <FooterLink href="/">Start</FooterLink>
            <FooterLink href="/history">History</FooterLink>
          </LinkGroup>
          <LinkGroup header="Some more links">
            <FooterLink href="/">Start</FooterLink>
            <FooterLink href="/history">History</FooterLink>
          </LinkGroup>
          <LinkGroup header="Some more links">
            <FooterLink href="/">Start</FooterLink>
            <FooterLink href="/history">History</FooterLink>
          </LinkGroup>
          <LinkGroup header="Some more links">
            <FooterLink href="/">Start</FooterLink>
            <FooterLink href="/history">History</FooterLink>
          </LinkGroup>
          <p className="col-span-full text-center text-xs">
            <Link
              className="hover:text-white hover:underline"
              href="https://github.com/akeamc/afbostader"
            >
              GitHub: akeamc/afbostader
            </Link>
          </p>
        </div>
      </footer>
    </div>
  );
}
