"use client";

import { useQuery } from "@tanstack/react-query";
import { Product } from "./af";

export function useVacancies() {
  return useQuery<Product[]>({
    queryKey: ["vacancies"],
    queryFn: () => fetch("/api/vacancies").then((res) => res.json()),
    refetchInterval: 10000,
  });
}
