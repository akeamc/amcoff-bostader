import Header from "@/components/Header";
import { VacancyContextProvider } from "@/components/VacancyContext";
import VacancyMap from "@/components/VacancyMap";
import VacancyTable from "@/components/VacancyTable";

export default async function Home() {
  return (
    <main className="grid grid-cols-2 overflow-hidden h-screen">
      <div className="row-span-1 col-span-2">
        <Header />
      </div>
      <VacancyContextProvider>
        <div className="overflow-auto">
          <VacancyTable />
        </div>
        <VacancyMap />
      </VacancyContextProvider>
    </main>
  );
}
