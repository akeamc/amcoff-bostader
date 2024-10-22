"use client";

import { PropsWithChildren, useState } from "react";
import {
  Field,
  Input,
  Label,
  Popover,
  PopoverBackdrop,
  PopoverButton,
  PopoverPanel,
} from "@headlessui/react";
import { useBreakpoint } from "@/lib/hooks/useBreakpoints";
import { useAuth } from "./AuthContext";

export default function LoginButton({
  className,
  children,
}: PropsWithChildren<{ className?: string }>) {
  const { isSm } = useBreakpoint("sm");
  const { loginMutation } = useAuth();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [invalidCredentials, setInvalidCredentials] = useState(false);

  async function onSubmit(formData: FormData) {
    setInvalidCredentials(false);

    const res = await loginMutation.mutateAsync({
      email: formData.get("email")!.toString(),
      password: formData.get("password")!.toString(),
    });

    setInvalidCredentials(res === "invalid-credentials");
  }

  return (
    <Popover>
      <PopoverButton className={className}>{children}</PopoverButton>
      <PopoverBackdrop
        transition
        className="fixed inset-0 bg-black/15 transition duration-100 ease-out data-[closed]:opacity-0 sm:hidden"
      />
      <PopoverPanel
        transition
        focus
        modal={!isSm}
        anchor="bottom end"
        className="border bg-white text-sm shadow-lg transition duration-200 ease-in-out [--anchor-gap:4px] data-[closed]:translate-y-4 data-[closed]:opacity-0 max-sm:!fixed max-sm:!inset-x-0 max-sm:!top-auto max-sm:bottom-0 max-sm:rounded-t-2xl sm:w-80 sm:rounded-xl sm:data-[closed]:-translate-y-1"
      >
        <form action={onSubmit} className="space-y-4 p-3">
          <p className="text-neutral-700">
            Använd samma inloggningsuppgifter som på afbostader.se.
          </p>

          <Field>
            <Label className="text-sm font-medium">E-postadress</Label>
            <Input
              required
              name="email"
              type="email"
              className="mt-2 block w-full rounded-lg border bg-neutral-50 px-3 py-1.5 text-sm focus:outline-none data-[focus]:outline-2 data-[focus]:-outline-offset-2 data-[focus]:outline-green-500"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              invalid={invalidCredentials}
            />
          </Field>
          <Field>
            <Label className="text-sm font-medium">Lösenord</Label>
            <Input
              required
              name="password"
              type="password"
              className="mt-2 block w-full rounded-lg border bg-neutral-50 px-3 py-1.5 text-sm focus:outline-none data-[focus]:outline-2 data-[focus]:-outline-offset-2 data-[focus]:outline-green-500"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              invalid={invalidCredentials}
            />
          </Field>
          {invalidCredentials && (
            <p className="rounded-lg border border-red-300 bg-red-50 p-4 text-red-500">
              Felaktigt användarnamn eller lösenord.
            </p>
          )}
          <button
            type="submit"
            disabled={loginMutation.isPending}
            className="block w-full rounded-lg bg-gradient-to-tr from-green-500 to-green-400 py-2 text-sm font-semibold text-white shadow disabled:opacity-50"
          >
            Logga in
          </button>
        </form>
      </PopoverPanel>
    </Popover>
  );
}
