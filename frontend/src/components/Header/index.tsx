import Link from "next/link";
import { AvailableLink, NavLink } from "./NavLink";
import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query";
import AuthSection from "./AuthSection";

export default async function Header() {
  const queryClient = new QueryClient();

  // await queryClient.prefetchQuery({
  //   queryKey: ['vacancies'],
  //   queryFn: listVacancies,
  // });

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <header className="mx-auto flex w-full max-w-screen-xl items-center p-4">
        <div className="flex flex-1 justify-center">
          <Link className="mr-auto font-serif text-xl font-medium" href="/">
            A<span className="text-neutral-400">(mcof)</span>F{" "}
            <span className="max-sm:hidden">Bostäder</span>
          </Link>
        </div>
        <div className="flex flex-1 justify-center">
          <nav className="flex items-center rounded-full border px-3 shadow-sm">
            <AvailableLink />
          </nav>
        </div>
        <div className="flex flex-1 justify-center">
          <div className="ml-auto">
            <AuthSection />
          </div>
        </div>
      </header>
    </HydrationBoundary>
  );
}
