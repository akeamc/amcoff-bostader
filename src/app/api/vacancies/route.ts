import { listVacant } from "@/lib/af";

export async function GET() {
  return Response.json(await listVacant());
}
