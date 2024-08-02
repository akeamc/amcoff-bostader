export default function VacancyFloorplan({id}: {id: number}) {
  return (
    <img src={`http://localhost:8000/vacancies/${id}/floorplan`} />
  )
}
