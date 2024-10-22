import AreaPictures from "@/components/AreaPictures";
import FloorplanButton from "@/components/floorplan/FloorplanButton";
import { getVacancy } from "@/lib/af";

export default async function VacancyPage({
  params,
}: {
  params: { id: number };
}) {
  const property = await getVacancy(parseInt(params.id.toString()));
  // const afUrl = `https://www.afbostader.se/lediga-bostader/bostadsdetalj/?obj=${encodeURIComponent(property.id)}&area=${encodeURIComponent(property.area)}&mode=0`;

  return (
    <main className="mx-auto w-full max-w-screen-lg p-4">
      <h1 className="text-2xl font-semibold tracking-tight sm:text-3xl">
        {property.short_description}, {property.area}
      </h1>
      <p className="text-neutral-700">{property.address.street}</p>
      <AreaPictures area={property.area} />
      <FloorplanButton id={params.id} />
      {/* <QueuePosition id={params.id} /> */}
    </main>
  );
}
