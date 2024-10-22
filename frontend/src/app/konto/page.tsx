"use client";

import { useAuth } from "@/components/AuthContext";
import { useUser } from "@/lib/hooks";
import { Button } from "@headlessui/react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function AccountPage() {
  const router = useRouter();
  const { data, isLoading } = useUser();
  const { logoutMutation } = useAuth();

  useEffect(() => {
    if (data === "unauthenticated") {
      router.push("/");
    }
  }, [data, router]);

  if (isLoading) {
    return null;
  }

  if (data === "unauthenticated") {
    return null;
  }

  return (
    <main className="mx-auto w-full max-w-screen-lg p-4">
      <h1 className="text-2xl font-semibold tracking-tight sm:text-3xl">
        Hej {data?.first_name}!
      </h1>
      <Button
        className="mt-8 rounded-md bg-red-500 px-4 py-2 text-sm font-medium text-white shadow-sm focus:outline-none data-[hover]:bg-red-600"
        onClick={() => logoutMutation.mutate()}
      >
        Logga ut
      </Button>
    </main>
  );
}
