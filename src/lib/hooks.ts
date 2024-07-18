"use client";

import { useQuery } from "@tanstack/react-query";
import { Property } from "./af";

export function useVacancies() {
  return useQuery<Property[]>({
    queryKey: ["vacancies"],
    queryFn: () =>
      fetch("/api/vacancies", { cache: "no-store" }).then((res) => res.json()),
    refetchInterval: 10000,
  });
}
