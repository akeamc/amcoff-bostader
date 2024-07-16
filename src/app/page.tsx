import VacancyMap from "@/components/VacancyMap";
import VacancyTable from "@/components/VacancyTable";

export default async function Home() {
  return (
    <main className="grid grid-cols-2 overflow-hidden h-screen">
      <div className="overflow-auto">
      <VacancyTable />
      </div>
      <VacancyMap />
    </main>
  );
}
