import { FilterProvider } from "@/components/FilterContext";
import VacancyGrid from "@/components/VacancyGrid";

export default async function Home() {
  return (
    <main className="mx-auto w-full max-w-screen-xl px-4">
      <FilterProvider>
        <VacancyGrid />
      </FilterProvider>
    </main>
  );
}
