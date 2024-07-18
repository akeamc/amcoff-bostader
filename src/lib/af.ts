export interface Address {
  street: string;
  city: string;
  postal_code: string;
}

export interface QueuePosition {
  position: number;
  total_in_queue: number;
}

export type PropertyType = "Apartment" | "Dorm";

export interface Property {
  id: number;
  property_type: PropertyType;
  area: string;
  description: string;
  short_description: string;
  address: Address;
  floor: number;
  size_sqm: number;
  reserved: boolean;
  queue_position: QueuePosition;
  rent: number;
  reserve_from: string;
  reserve_until: string;
}

export interface PropertyDetail extends Property {}

export interface Area {}

interface Picture {
  alt: string | null;
  url: string;
}

export interface AreaDetail extends Area {
  pictures: Picture[];
}

export function listVacancies(): Promise<Property[]> {
  return fetch("http://localhost:8000/vacancies", { cache: "default" }).then(
    (res) => res.json(),
  );
}

export function getVacancy(id: number): Promise<PropertyDetail> {
  return fetch(`http://localhost:8000/vacancies/${encodeURIComponent(id)}`, {
    cache: "default",
  }).then((res) => res.json());
}

export function getArea(areaName: string): Promise<AreaDetail> {
  return fetch(`http://localhost:8000/areas/${encodeURIComponent(areaName)}`, {
    cache: "default",
  }).then((res) => res.json());
}
