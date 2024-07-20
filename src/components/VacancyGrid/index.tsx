"use client";

import { useVacancies } from "@/lib/hooks";
import VacancyCell from "./VacancyCell";
import { Property } from "@/lib/af";
import Filter from "./Filter";
import {
  createColumnHelper,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  PaginationState,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";
import { useFilter } from "../FilterContext";

const columnHelper = createColumnHelper<Property>();

const columns = [
  columnHelper.accessor("queue_position.position", {
    id: "queue_position.position",
  }),
  columnHelper.accessor("rent", {}),
  columnHelper.accessor("size_sqm", {}),
  columnHelper.accessor("reserve_until", {}),
  columnHelper.accessor("move_in", {}),
  columnHelper.accessor("floor", {}),
];

export default function VacancyGrid() {
  const { data } = useVacancies();
  const { sorting, setSorting } = useFilter();

  const table = useReactTable({
    data: data || [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: setSorting,
    state: {
      sorting,
    },
  });

  return (
    <>
      <Filter />
      <div className="-m-2 grid grid-cols-3 gap-x-2 gap-y-4">
        {table.getRowModel().rows.map((row) => (
          <VacancyCell key={row.id} property={row.original} />
        ))}
      </div>
    </>
  );
}
