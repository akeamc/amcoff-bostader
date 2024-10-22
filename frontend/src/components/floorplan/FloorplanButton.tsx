"use client";

import {
  Button,
  Description,
  Dialog,
  DialogBackdrop,
  DialogPanel,
  DialogTitle,
} from "@headlessui/react";
import { useState } from "react";
import VacancyFloorplan from "./VacancyFloorplan";
import { useVacancy } from "@/lib/hooks";
import { formatPostalCode } from "@/lib/utils";

export default function FloorplanButton({ id }: { id: number }) {
  const { data } = useVacancy(id);
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      <Button
        onClick={() => setIsOpen(true)}
        className="rounded-md border bg-white px-4 py-2 text-sm font-medium shadow-sm focus:outline-none data-[hover]:bg-neutral-100 data-[focus]:outline-1 data-[focus]:outline-white"
      >
        Planritning
      </Button>

      <Dialog
        open={isOpen}
        as="div"
        className="relative z-10 focus:outline-none"
        onClose={() => setIsOpen(false)}
      >
        <DialogBackdrop transition className="fixed inset-0 bg-black/15" />
        <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4">
            <DialogPanel
              transition
              className="data-[closed]:transform-[scale(95%)] w-full max-w-screen-lg rounded-xl bg-white p-6 duration-300 ease-out data-[closed]:opacity-0"
            >
              <DialogTitle as="h3" className="mb-4">
                <div className="text-lg font-medium">
                  {data?.address.street}
                </div>
                <div className="text-sm text-neutral-700">
                  {formatPostalCode(data?.address.postal_code)}{" "}
                  {data?.address.city}
                </div>
              </DialogTitle>
              <VacancyFloorplan id={id} className="mx-auto" />
            </DialogPanel>
          </div>
        </div>
      </Dialog>
    </>
  );
}
