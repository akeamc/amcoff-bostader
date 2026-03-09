import type { PageLoad } from './$types';
import { listVacancies, getArea } from '$lib/api';

export const load: PageLoad = async () => {
	let vacancies: Awaited<ReturnType<typeof listVacancies>> = [];
	let areas: Record<string, Awaited<ReturnType<typeof getArea>>> = {};

	try {
		vacancies = await listVacancies();

		// Get unique area names and fetch area data
		const areaNames = Array.from(new Set(vacancies.map((v) => v.area)));
		const areaResults = await Promise.allSettled(areaNames.map((name) => getArea(name)));

		areaNames.forEach((name, i) => {
			const result = areaResults[i];
			if (result.status === 'fulfilled') {
				areas[name] = result.value;
			}
		});
	} catch {
		// API not available; data will be loaded client-side
	}

	return { vacancies, areas };
};
