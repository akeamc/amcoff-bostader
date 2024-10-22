import { API_URL } from "@/lib/af";

export default function VacancyFloorplan({
  id,
  className,
}: {
  id: number;
  className?: string;
}) {
  return (
    // eslint-disable-next-line @next/next/no-img-element
    <img
      src={`${API_URL}/vacancies/${id}/floorplan`}
      alt="Planritning över bostaden"
      className={className}
    />
  );
}
