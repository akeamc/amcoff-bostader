"use client";

import { Picture } from "@/lib/af";
import { useArea } from "@/lib/hooks";
import classNames from "classnames";
import Image from "next/image";

function AreaPicture({ picture, span }: { picture?: Picture; span: 1 | 2 }) {
  const className = classNames(
    "bg-neutral-100 rounded-md sm:rounded-xl object-cover size-full aspect-square",
    {
      "row-span-1 col-span-1": span === 1,
      "row-span-2 col-span-2": span === 2,
    },
  );
  const size = 500 * span;

  if (!picture) return <div className={className} />;

  return (
    <Image
      className={className}
      src={picture?.url}
      alt={picture?.alt || ""}
      width={size}
      height={size}
    />
  );
}

export default function AreaPictures(props: { area?: string }) {
  const { data: area } = useArea(props.area);
  const pictures = area?.pictures;

  return (
    <div className="sm:gap-3p my-4 grid grid-cols-4 grid-rows-2 gap-2">
      <AreaPicture picture={pictures?.[0]} span={2} />
      <AreaPicture picture={pictures?.[1]} span={1} />
      <AreaPicture picture={pictures?.[2]} span={1} />
      <AreaPicture picture={pictures?.[3]} span={1} />
      <AreaPicture picture={pictures?.[4]} span={1} />
    </div>
  );
}
