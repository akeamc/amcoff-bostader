import { getVacant } from "@/lib/af";

export async function GET() {
  return Response.json(await getVacant());
}
