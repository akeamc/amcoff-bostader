import type { PageLoad } from './$types';
import { getVacancy, getArea } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
	const id = parseInt(params.id);
	try {
		const property = await getVacancy(id);
		const area = await getArea(property.area).catch(() => null);
		return { property, area };
	} catch {
		// API not available; will show loading state client-side
		return { property: null, area: null };
	}
};
