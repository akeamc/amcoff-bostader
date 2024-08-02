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
      <header className="flex items-center p-4 mx-auto w-full max-w-screen-xl">
        <div className="flex flex-1 justify-center">
          <span className="mr-auto font-serif text-xl font-medium">
            A
            <Link
              className="text-neutral-400 hover:underline"
              href="https://amcoff.net"
            >
              (mcof)
            </Link>
            F <span className="max-sm:hidden">Bostäder</span>
          </span>
        </div>
        <div className="flex flex-1 justify-center">
          <nav className="flex items-center rounded-full border px-3 shadow-sm">
            <AvailableLink />
            <NavLink href="/history">History</NavLink>
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
