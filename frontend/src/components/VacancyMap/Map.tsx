"use client";

import { Address, API_URL, Property } from "@/lib/af";
import { useVacancies } from "@/lib/hooks";
import { useQuery } from "@tanstack/react-query";
import L from "leaflet";
import "leaflet/dist/leaflet.css";

import { MapContainer, Marker, Popup, TileLayer } from "react-leaflet";

interface Place {
  lat: number;
  lon: number;
}

function usePlace(address: Address) {
  return useQuery<Place>({
    queryKey: [
      "geo",
      {
        street: address.street,
        postalcode: address.postal_code,
        city: address.city,
      },
    ],
    queryFn: async () => {
      const data = await fetch(
        `${API_URL}/geocode?street=${encodeURIComponent(address.street)}&postalcode=${address.postal_code}&city=${address.city}`,
        { cache: "force-cache" },
      ).then((res) => res.json());
      const place = data?.[0];
      if (!place) throw new Error("no place");
      return place;
    },
    refetchOnWindowFocus: false,
    retry: true,
  });
}

function VacancyMarker({ product }: { product: Property }) {
  const { data } = usePlace(product.address);

  if (!data) return null;

  const icon = L.divIcon({
    className:
      "bg-green-500 rounded-full border-2 border-white size-8 -mt-4 -ml-4",
    iconSize: null as any,
  });

  return (
    <Marker position={[data.lat, data.lon]} icon={icon}>
      <Popup>
        <strong>{product.area}:</strong> {product.description}
      </Popup>
    </Marker>
  );
}

export default function Map() {
  const { data } = useVacancies();

  return (
    <MapContainer
      center={[55.704261, 13.1915074]}
      zoom={14}
      scrollWheelZoom
      style={{ width: "100%", height: "100%" }}
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://tiles.stadiamaps.com/tiles/alidade_smooth/{z}/{x}/{y}.png"
      />
      {data?.map((p) => <VacancyMarker key={p.id} product={p} />)}
    </MapContainer>
  );
}
