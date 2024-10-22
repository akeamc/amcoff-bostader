"use client";

import { useVacancy } from "@/lib/hooks";

export default function QueuePosition({ id }: { id: number }) {
  const { data } = useVacancy(id);

  return (
    <div className="rounded-lg border p-4 text-xl">
      {data?.queue_position.position === null ? (
        <>{data?.queue_position.total_in_queue} i kö</>
      ) : (
        <>
          {data?.queue_position.position} av{" "}
          {data?.queue_position.total_in_queue}
        </>
      )}
    </div>
  );
}
