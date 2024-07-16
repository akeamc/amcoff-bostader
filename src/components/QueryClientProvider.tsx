"use client";

import {
  QueryClient,
  QueryClientProvider as ActualQueryClientProvider,
} from "@tanstack/react-query";
import { PropsWithChildren } from "react";

const queryClient = new QueryClient();

export default function QueryClientProvider({
  children,
}: PropsWithChildren<{}>) {
  return (
    <ActualQueryClientProvider client={queryClient}>
      {children}
    </ActualQueryClientProvider>
  );
}
