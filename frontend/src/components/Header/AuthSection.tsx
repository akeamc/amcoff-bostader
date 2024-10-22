"use client";

import Link from "next/link";
import { useAuth } from "../AuthContext";
import LoginButton from "../LoginButton";

function Avatar() {
  const { user } = useAuth();

  if (!user) return <div className="size-8 rounded-full bg-neutral-300" />;

  return (
    <Link
      href="/konto"
      className="text-md flex size-8 items-center justify-center rounded-full bg-green-500 leading-none text-white"
    >
      {user.first_name.substring(0, 1)}
      {user.last_name.substring(0, 1)}
    </Link>
  );
}

export default function AuthSection() {
  const { isAuthenticated } = useAuth();

  if (typeof isAuthenticated === "undefined") {
    return null;
  }

  if (isAuthenticated) {
    return <Avatar />;
  }

  return (
    <LoginButton className="group rounded-full border p-1 text-sm font-semibold shadow-sm">
      <div className="rounded-full bg-gradient-to-tr from-green-500 to-green-400 px-4 py-1 text-white transition-all group-hover:from-green-600 group-hover:to-green-500">
        Logga in
      </div>
    </LoginButton>
  );
}
