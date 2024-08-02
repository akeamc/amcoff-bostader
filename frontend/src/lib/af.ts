export interface Address {
  street: string;
  city: string;
  postal_code: string;
}

export interface QueuePosition {
  position: number | null;
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
  move_in: string;
}

export interface PropertyDetail extends Property {
  facing: string;
}

export interface Area {}

export interface Picture {
  alt: string | null;
  url: string;
}

export interface AreaDetail extends Area {
  pictures: Picture[];
}

export function listVacancies(): Promise<Property[]> {
  return fetch("http://localhost:8000/vacancies", {
    cache: "default",
    credentials: "include",
  }).then((res) => res.json());
}

export function getVacancy(id: number): Promise<PropertyDetail> {
  return fetch(`http://localhost:8000/vacancies/${encodeURIComponent(id)}`, {
    cache: "default",
    credentials: "include",
  }).then((res) => res.json());
}

export function getArea(areaName: string): Promise<AreaDetail> {
  return fetch(`http://localhost:8000/areas/${encodeURIComponent(areaName)}`, {
    cache: "default",
  }).then((res) => res.json());
}

export interface EmailPassword {
  email: string;
  password: string;
}

export type LoginResponse = UserDetails | "invalid-credentials";

export interface UserDetails {
  first_name: String;
  last_name: String;
}

export async function getUser(): Promise<UserDetails | "unauthenticated"> {
  const res = await fetch("http://localhost:8000/user", {
    cache: "default",
    credentials: "include",
  });

  if (res.ok) return res.json();

  return "unauthenticated";
}

export async function login(details: EmailPassword): Promise<LoginResponse> {
  const res = await fetch(`http://localhost:8000/login`, {
    headers: {
      "content-type": "application/json",
    },
    method: "POST",
    body: JSON.stringify(details),
    credentials: "include",
  });

  if (res.ok) {
    return res.json();
  } else if (res.status === 403) {
    return "invalid-credentials";
  } else {
    throw new Error(`unexpected response from server: ${await res.text()}`);
  }
}

export async function logout(): Promise<Response> {
  return await fetch("http://localhost:8000/logout", {cache: "default", credentials: "include"});
}
