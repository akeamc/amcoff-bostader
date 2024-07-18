"use client";

import { Property } from "@/lib/af";
import { useVacancies } from "@/lib/hooks";
import {
  Column,
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  RowData,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";
import { useEffect, useState } from "react";
import VacancyRow from "./VacancyRow";

const CSN = 13_156;

declare module "@tanstack/react-table" {
  //allows us to define custom properties for our columns
  interface ColumnMeta<TData extends RowData, TValue> {
    filterVariant?: "text" | "range" | "select";
  }
}
const columnHelper = createColumnHelper<Property>();

const columns = [
  columnHelper.accessor("queue_position.position", {
    header: "plats",
  }),
  columnHelper.accessor("queue_position.total_in_queue", {
    header: "tot.",
  }),
  columnHelper.accessor("reserved", {
    cell: (info) => (info.getValue() ? "y" : ""),
  }),
  columnHelper.accessor("rent", {
    cell: (info) => {
      const rent = info.getValue();

      return (
        <>
          {rent.toLocaleString("sv")} kr (
          {((100 * rent) / CSN).toLocaleString("sv", {
            minimumFractionDigits: 1,
            maximumFractionDigits: 1,
          })}{" "}
          %)
        </>
      );
    },
  }),
  columnHelper.accessor("area", { meta: { filterVariant: "text" } }),
  columnHelper.accessor("floor", {}),
  columnHelper.accessor("description", {}),
  columnHelper.accessor("size_sqm", {
    cell: (info) =>
      info.getValue().toLocaleString("sv", {
        maximumFractionDigits: 1,
        minimumFractionDigits: 1,
      }),
  }),
  columnHelper.accessor("address.street", {}),
  columnHelper.accessor("property_type", {
    meta: {
      filterVariant: "select",
    },
  }),
];

export default function VacancyTable() {
  const { data, dataUpdatedAt, isFetching } = useVacancies();

  const [sorting, setSorting] = useState<SortingState>([
    {
      id: "queue_position.position",
      desc: false,
    },
  ]);

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
    <div className="p-2">
      <p>
        {new Date(dataUpdatedAt).toLocaleString("sv", {
          hour: "2-digit",
          minute: "2-digit",
          second: "2-digit",
        })}{" "}
        {isFetching ? "..." : null}
      </p>
      <div className="h-2" />
      <table>
        <thead>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <th key={header.id} colSpan={header.colSpan}>
                    {header.isPlaceholder ? null : (
                      <>
                        <div
                          className={
                            header.column.getCanSort()
                              ? "cursor-pointer select-none"
                              : ""
                          }
                          onClick={header.column.getToggleSortingHandler()}
                          title={
                            header.column.getCanSort()
                              ? header.column.getNextSortingOrder() === "asc"
                                ? "Sort ascending"
                                : header.column.getNextSortingOrder() === "desc"
                                  ? "Sort descending"
                                  : "Clear sort"
                              : undefined
                          }
                        >
                          {flexRender(
                            header.column.columnDef.header,
                            header.getContext(),
                          )}
                          {{
                            asc: " 🔼",
                            desc: " 🔽",
                          }[header.column.getIsSorted() as string] ?? null}
                        </div>
                        {header.column.getCanFilter() ? (
                          <div>
                            <Filter column={header.column} />
                          </div>
                        ) : null}
                      </>
                    )}
                  </th>
                );
              })}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => {
            return <VacancyRow key={row.id} row={row} />;
          })}
        </tbody>
      </table>
      <div>{table.getRowModel().rows.length.toLocaleString()} Rows</div>
    </div>
  );
}

function Filter({ column }: { column: Column<Property, unknown> }) {
  const columnFilterValue = column.getFilterValue();
  const { filterVariant } = column.columnDef.meta || {};

  return filterVariant === "range" ? (
    <div>
      <div className="flex space-x-2">
        {/* See faceted column filters example for min max values functionality */}
        <DebouncedInput
          type="number"
          value={(columnFilterValue as [number, number])?.[0] ?? ""}
          onChange={(value) =>
            column.setFilterValue((old: [number, number]) => [value, old?.[1]])
          }
          placeholder={`Min`}
          className="w-24 rounded border shadow"
        />
        <DebouncedInput
          type="number"
          value={(columnFilterValue as [number, number])?.[1] ?? ""}
          onChange={(value) =>
            column.setFilterValue((old: [number, number]) => [old?.[0], value])
          }
          placeholder={`Max`}
          className="w-24 rounded border shadow"
        />
      </div>
      <div className="h-1" />
    </div>
  ) : filterVariant === "select" ? (
    <select
      onChange={(e) => column.setFilterValue(e.target.value)}
      value={columnFilterValue?.toString()}
    >
      {/* See faceted column filters example for dynamic select options */}
      <option value="">All</option>
      <option value="Apartment">Apartment</option>
      <option value="Dorm">Dorm</option>
    </select>
  ) : filterVariant === "text" ? (
    <DebouncedInput
      className="w-36 rounded border shadow"
      onChange={(value) => column.setFilterValue(value)}
      placeholder={`Search...`}
      type="text"
      value={(columnFilterValue ?? "") as string}
    />
  ) : // See faceted column filters example for datalist search suggestions
  null;
}

// A typical debounced input react component
function DebouncedInput({
  value: initialValue,
  onChange,
  debounce = 500,
  ...props
}: {
  value: string | number;
  onChange: (value: string | number) => void;
  debounce?: number;
} & Omit<React.InputHTMLAttributes<HTMLInputElement>, "onChange">) {
  const [value, setValue] = useState(initialValue);

  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  useEffect(() => {
    const timeout = setTimeout(() => {
      onChange(value);
    }, debounce);

    return () => clearTimeout(timeout);
  }, [value]);

  return (
    <input
      {...props}
      value={value}
      onChange={(e) => setValue(e.target.value)}
    />
  );
}
