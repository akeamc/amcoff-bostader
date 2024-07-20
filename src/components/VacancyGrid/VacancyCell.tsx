"use client";

import { Property, QueuePosition } from "@/lib/af";
import { useArea, useVacancy } from "@/lib/hooks";
import Link from "next/link";
import { useEffect, useState } from "react";
import { Temporal } from "@js-temporal/polyfill";
import classNames from "classnames";
import Image from "next/image";
import { Key } from "react-feather";

function QueueInfo(props: { position?: QueuePosition; className?: string }) {
  const { position, total_in_queue } = props.position || {};

  return (
    <div
      className={classNames(
        "inline-block rounded-[999px] bg-white px-2 py-1 text-sm text-black shadow-md",
        { "bg-gradient-to-tr from-yellow-500 to-yellow-400": position === 1 },
        props.className,
      )}
      title="Köplats"
    >
      {position} av {total_in_queue}
    </div>
  );
}

function ReserveableSpan(props: { from?: string; until?: string }) {
  const [urgent, setUrgent] = useState(false);
  const from = Temporal.PlainDate.from(props.from || "").toZonedDateTime({
    timeZone: "Europe/Stockholm",
    plainTime: Temporal.PlainTime.from("00:00:00.000"),
  });
  const until = Temporal.PlainDate.from(props.until || "").toZonedDateTime({
    timeZone: "Europe/Stockholm",
    plainTime: Temporal.PlainTime.from("23:59:59.999"),
  });

  useEffect(() => {
    check();

    function check() {
      const seconds = Temporal.Now.instant().until(until.toInstant(), {
        smallestUnit: "second",
      }).seconds;
      setUrgent(seconds < 86_400);
    }

    const interval = setInterval(check, 1000);

    return () => clearInterval(interval);
  }, [until]);

  if (!props.from || !props.until) return;

  const sameYear = from.year === until.year;
  const sameYearAndMonth = sameYear && from.month === until.month;
  const currentYear =
    sameYear && from.year === Temporal.Now.zonedDateTimeISO().year;

  return (
    <span className={classNames({ "font-medium text-red-500": urgent })}>
      Anmälan{" "}
      <time>
        {from.toLocaleString("sv", {
          year: sameYear ? undefined : "numeric",
          month: sameYearAndMonth ? undefined : "short",
          day: "numeric",
        })}
      </time>
      {sameYearAndMonth ? "–" : " – "}
      <time>
        {until.toLocaleString("sv", {
          year: currentYear ? undefined : "numeric",
          month: "short",
          day: "numeric",
        })}
      </time>
    </span>
  );
}

function AccessDate(props: { date?: string }) {
  const [currentYear, setCurrentYear] = useState<number>();

  useEffect(() => {
    setCurrentYear(Temporal.Now.zonedDateTimeISO().year);
  }, []);

  const date = Temporal.PlainDate.from(props.date || "");

  return (
    <span>
      Tillträde{" "}
      {date.toLocaleString("sv", {
        year: currentYear === date.year ? undefined : "numeric",
        month: "short",
        day: "numeric",
      })}
    </span>
  );
}

export default function VacancyCell(props: { property: Property }) {
  const { data: property } = useVacancy(props.property.id);
  const { data: area } = useArea(props.property.area);

  const picture = area?.pictures?.[0];

  return (
    <Link href="/" className="group">
      <article className="flex flex-col gap-1 rounded-2xl p-2 leading-tight text-neutral-700 group-hover:bg-gray-100">
        <div className="relative mb-3 aspect-video w-full overflow-hidden rounded-2xl bg-neutral-100 shadow-sm">
          {picture && (
            <Image
              src={picture.url}
              alt={picture.alt || ""}
              width={500}
              height={500}
              className="size-full object-cover transition-transform group-hover:scale-105"
            />
          )}
          <QueueInfo
            className="absolute right-2 top-2"
            position={property?.queue_position}
          />
        </div>
        <h3 className="text-lg font-medium leading-none tracking-tight text-black">
          {property?.area}
        </h3>
        <div>
          {property?.short_description.toLocaleLowerCase()} ⋅{" "}
          {property?.size_sqm.toLocaleString("sv", {
            minimumFractionDigits: 1,
            maximumFractionDigits: 1,
          })}{" "}
          m<sup>2</sup> ⋅ vån {property?.floor}
        </div>
        <div>
          <ReserveableSpan
            from={property?.reserve_from}
            until={property?.reserve_until}
          />
          {" ⋅ "}
          <AccessDate date={property?.move_in} />
        </div>
        <div className="font-medium text-black">
          {property?.rent.toLocaleString("sv")}&nbsp;kr/mån
        </div>
      </article>
    </Link>
  );
}
