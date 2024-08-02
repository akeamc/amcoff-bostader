"use client";

import { useAuth } from "../AuthContext";
import LoginButton from "../LoginButton";

function Avatar() {
  const { user, logoutMutation } = useAuth();

  if (!user) return <div className="size-8 rounded-full bg-neutral-300" />;

  return (
    <button onClick={() => logoutMutation.mutate()} className="size-8 rounded-full bg-green-500 text-white flex items-center justify-center text-lg leading-none">
      {user.first_name.substring(0, 1)}
    </button>
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
