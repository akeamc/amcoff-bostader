"use client";

import { Check, ChevronDown } from "react-feather";
import { Sorting, useFilter } from "../FilterContext";
import {
  Checkbox,
  Menu,
  MenuButton,
  MenuItem,
  MenuItems,
} from "@headlessui/react";

const SORTING_NAMES: Record<string, string> = {
  "queue_position.position": "Köplats",
  rent: "Hyra",
  size_sqm: "Golvyta",
  reserve_until: "Sista anmälningsdag",
  move_in: "Inflyttning",
  floor: "Våning",
};

function SortingDropdown() {
  const { sorting: currentSorting, setSorting } = useFilter();

  return (
    <div className="text-sm text-gray-700">
      Sortera efter
      <Menu as="div" className="relative ml-2 inline-block text-left">
        <div>
          <MenuButton className="inline-flex w-full justify-center gap-x-1.5 rounded-lg px-3 py-2 text-sm font-semibold text-gray-900 hover:bg-gray-100">
            {SORTING_NAMES[currentSorting[0].id]}
            <ChevronDown aria-hidden className="-mr-1 h-5 w-5 text-gray-400" />
            {/* <ChevronDownIcon aria-hidden="true" className="-mr-1 h-5 w-5 text-gray-400" /> */}
          </MenuButton>
        </div>

        <MenuItems
          transition
          className="absolute right-0 z-10 mt-2 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 transition focus:outline-none data-[closed]:scale-95 data-[closed]:transform data-[closed]:opacity-0 data-[enter]:duration-100 data-[leave]:duration-75 data-[enter]:ease-out data-[leave]:ease-in"
        >
          <div className="py-1">
            {Object.entries(SORTING_NAMES).map(([id, name]) => (
              <MenuItem key={id}>
                <button
                  onClick={() =>
                    setSorting([
                      {
                        id,
                        desc: false,
                      },
                    ])
                  }
                  className="block w-full px-4 py-2 text-start text-sm text-gray-700 data-[focus]:bg-gray-100 data-[focus]:text-gray-900"
                >
                  {name}
                </button>
              </MenuItem>
            ))}
          </div>
        </MenuItems>
      </Menu>
    </div>
  );
}

export default function Filter() {
  return (
    <div className="mb-4 flex justify-between border-b pb-1">
      <SortingDropdown />
    </div>
  );
}
