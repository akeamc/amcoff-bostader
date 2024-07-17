"use client";

import { useQuery } from "@tanstack/react-query";
import dynamic from "next/dynamic";

const Chart = dynamic(() => import("react-apexcharts"), { ssr: false });

interface Data {
  categories: number[];
  series: {
    name: string;
    data: (number | null)[];
  }[];
}

export default function QueueChart() {
  const max = 10;

  const { data } = useQuery<Data>({
    queryKey: ["archive", { max }],
    queryFn: () =>
      fetch(`http://localhost:8000/archive?max=${max}`, {
        cache: "no-cache",
      }).then((res) => res.json()),
    refetchInterval: 5000,
  });

  if (!data) return;

  return (
    <Chart
      options={{
        chart: {
          id: "apexchart-example",
        },
        xaxis: {
          type: "datetime",
          categories: data.categories,
          labels: {
            formatter: (v) =>
              new Date(+v * 1000).toLocaleDateString("sv", {
                hour: "2-digit",
                minute: "2-digit",
                day: "numeric",
                month: "short",
              }),
          },
          tickPlacement: "on",
        },
        yaxis: {
          min: 0,
          max,
        },
        legend: {
          show: false,
        },
        tooltip: {
          // shared: false,
        },
      }}
      series={data.series}
      type="line"
      width="1800"
      height="1000"
    />
  );
}
