"use client";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { getArea, getVacancy, listVacancies, Property, PropertyDetail } from "./af";
import { useMemo } from "react";

export function useVacancies() {
  return useQuery<Property[]>({
    queryKey: ["vacancies"],
    queryFn: listVacancies,
    refetchInterval: 30_000,
  });
}

export function useVacancy(id: number) {
  const queryClient = useQueryClient();

  return useQuery<PropertyDetail | Property>({
    queryKey: ["vacancies", id],
    queryFn: () => getVacancy(id),
    refetchInterval: 30_000,
    placeholderData: () => {
      return queryClient
        .getQueryData<Property[]>(["vacancies"])
        ?.find((p) => p.id === id);
    },
  });
}

export function useAreaNames() {
  const { data: vacancies } = useVacancies();

  return useMemo(() => {
    if (!vacancies) return undefined;

    return Array.from(new Set(vacancies.map((v) => v.area))).sort((a, b) =>
      a.localeCompare(b),
    );
  }, [vacancies]);
}

export function useArea(name?: string) {
  return useQuery({
    queryKey: ["areas", name],
    enabled: !!name,
    queryFn: () => getArea(name!),
  });
}
