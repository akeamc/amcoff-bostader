"use client";

import { useVacancies } from "@/lib/hooks";
import VacancyCell from "./VacancyCell";

export default function VacancyGrid() {
  const { data } = useVacancies();

  return (
    <div className="grid w-full grid-cols-3 gap-x-2 gap-y-4">
      {data?.map((property) => (
        <VacancyCell key={property.id} property={property} />
      ))}
    </div>
  );
}
