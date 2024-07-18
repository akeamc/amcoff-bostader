import Header from "@/components/Header";
import { VacancyContextProvider } from "@/components/VacancyContext";
import VacancyMap from "@/components/VacancyMap";
import VacancyTable from "@/components/VacancyTable";

export default async function Home() {
  return (
    <main className="grid grid-cols-2">
      <VacancyContextProvider>
        <div className="overflow-auto">
          <VacancyTable />
        </div>
        <VacancyMap />
      </VacancyContextProvider>
    </main>
  );
}
