import Link from "next/link";

export default function Header() {
  return (
    <header>
      <nav>
        <Link href="/">Start</Link>
        <Link href="/history">History</Link>
      </nav>
    </header>
  );
}
