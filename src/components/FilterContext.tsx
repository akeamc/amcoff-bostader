"use client";

import { Property } from "@/lib/af";
import { ColumnSort, SortingState } from "@tanstack/react-table";
import {
  createContext,
  Dispatch,
  PropsWithChildren,
  SetStateAction,
  useContext,
  useState,
} from "react";

export type Sorting = "queue" | "rent";

export interface FilterContextProps {
  sorting: SortingState;
  setSorting: Dispatch<SetStateAction<SortingState>>;
  vacancies?: Property[];
}

export const FilterContext = createContext<FilterContextProps>({
  sorting: [
    {
      id: "queue_position.position",
      desc: false,
    },
  ],
  setSorting: () => {
    throw new Error("unimplemented");
  },
});

export const useFilter = () => useContext(FilterContext);

export function FilterProvider({ children }: PropsWithChildren<{}>) {
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: "queue_position.position",
      desc: false,
    },
  ]);

  return (
    <FilterContext.Provider value={{ sorting, setSorting }}>
      {children}
    </FilterContext.Provider>
  );
}
