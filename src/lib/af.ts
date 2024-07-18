export interface Address {
  street: string;
  city: string;
  postal_code: string;
}

export interface QueuePosition {
  position: number;
  total_in_queue: number;
}

export interface Property {
  id: string;
  property_type: string;
  area: string;
  description: string;
  shortDescription: string;
  address: Address;
  floor: number;
  size_sqm: number;
  reserved: boolean;
  queue_position: QueuePosition;
  rent: string;
}

export async function listVacant(): Promise<Property[]> {
  const res = await fetch("http://localhost:8000/vacant").then((res) =>
    res.json(),
  );

  return res;
}
