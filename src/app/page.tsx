import VacancyGrid from "@/components/VacancyGrid";
import { listVacancies } from "@/lib/af";

export default async function Home() {
  const vacancies = listVacancies();

  return (
    <main className="mx-auto w-full max-w-screen-xl px-4">
      <VacancyGrid />
    </main>
  );
}
