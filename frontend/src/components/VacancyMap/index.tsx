"use client";

import dynamic from "next/dynamic";

const VacancyMap = dynamic(() => import("./Map"), {
  ssr: false,
  loading: () => <p>Loading...</p>,
});

export default VacancyMap;
