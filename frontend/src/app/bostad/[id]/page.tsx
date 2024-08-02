import AreaPictures from "@/components/AreaPictures";
import VacancyFloorplan from "@/components/VacancyFloorplan";
import { getVacancy } from "@/lib/af";
import Link from "next/link";

export default async function VacancyPage({
  params,
}: {
  params: { id: number };
}) {
  const property = await getVacancy(parseInt(params.id.toString()));
  const afUrl = `https://www.afbostader.se/lediga-bostader/bostadsdetalj/?obj=${encodeURIComponent(property.id)}&area=${encodeURIComponent(property.area)}&mode=0`;

  return (
    <main className="mx-auto w-full max-w-screen-lg p-4">
      <h1 className="text-2xl font-semibold tracking-tight sm:text-3xl">
        {property.short_description}
      </h1>
      <AreaPictures area={property.area} />
      <Link className="underline text-blue-600" href={afUrl}>Visa på AF Bostäder</Link>
      <VacancyFloorplan id={property.id} />
      <pre>{JSON.stringify(property, null, 2)}</pre>
    </main>
  );
}
