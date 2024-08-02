"use client";

import {
  login,
  EmailPassword,
  LoginResponse,
  UserDetails,
  getUser,
  logout,
} from "@/lib/af";
import {
  useMutation,
  UseMutationResult,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { createContext, PropsWithChildren, useContext } from "react";

interface AuthContextData {
  loginMutation: UseMutationResult<LoginResponse, any, EmailPassword>;
  logoutMutation: UseMutationResult<Response, any, void>;
  isAuthenticated: undefined | boolean;
  user: UserDetails | undefined;
}

export const AuthContext = createContext<AuthContextData | null>(null);

export function AuthProvider({ children }: PropsWithChildren<{}>) {
  const queryClient = useQueryClient();
  
  function invalidateQueries() {
    queryClient.invalidateQueries({
      predicate: (q) => q.queryKey[0] === "vacancies"
    });
  }

  const loginMutation = useMutation({
    mutationKey: ["login"],
    mutationFn: login,
    onSuccess: (data) => {
      if (typeof data === "object") {
        queryClient.setQueryData(["user"], data);
        invalidateQueries();
      }
    },
  });
  const logoutMutation = useMutation({
    mutationKey: ["logout"],
    mutationFn: logout,
    onSuccess: () => {
      queryClient.setQueryData(["user"], "unauthenticated");
      invalidateQueries();
    },
  })
  const { data, isPending } = useQuery({
    queryKey: ["user"],
    queryFn: getUser,
  });

  return (
    <AuthContext.Provider
      value={{
        loginMutation,
        logoutMutation,
        isAuthenticated: isPending
          ? undefined
          : data && data !== "unauthenticated",
        user: data !== "unauthenticated" ? data : undefined,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx)
    throw new Error("useAuth must be called from within an AuthProvider");
  return ctx;
};
