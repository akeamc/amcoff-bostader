"use client";

import { createContext, PropsWithChildren, useState } from "react";

export interface ContextData {
  highlighted: string | null;
  setHighlighted: (highlighted: string | null) => void;
}

export const VacancyContext = createContext<ContextData>({
  highlighted: null,
  setHighlighted: () => {
    throw new Error("unimplemented");
  },
});

export function VacancyContextProvider({ children }: PropsWithChildren<{}>) {
  const [highlighted, setHighlighted] = useState<string | null>(null);

  return (
    <VacancyContext.Provider value={{ highlighted, setHighlighted }}>
      {children}
    </VacancyContext.Provider>
  );
}
