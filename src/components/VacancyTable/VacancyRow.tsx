import { Property } from "@/lib/af";
import { flexRender, Row } from "@tanstack/react-table";

export default function VacancyRow({ row }: { row: Row<Property> }) {
  return (
    <tr>
      {row.getVisibleCells().map((cell) => {
        return (
          <td key={cell.id} className="border border-neutral-300 px-2">
            {flexRender(cell.column.columnDef.cell, cell.getContext())}
          </td>
        );
      })}
    </tr>
  );
}
