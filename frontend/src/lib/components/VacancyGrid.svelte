<script lang="ts">
	import type { Property, AreaDetail } from '$lib/api';
	import { sortField, SORT_NAMES, type SortField } from '$lib/stores';
	import VacancyCard from './VacancyCard.svelte';

	let {
		vacancies,
		areas
	}: { vacancies: Property[]; areas: Record<string, AreaDetail | undefined> } = $props();

	let showSortMenu = $state(false);

	function sortVacancies(items: Property[], field: SortField): Property[] {
		return [...items].sort((a, b) => {
			switch (field) {
				case 'queue_position': {
					const pa = a.queue_position.position;
					const pb = b.queue_position.position;
					if (pa === null && pb === null) return 0;
					if (pa === null) return 1;
					if (pb === null) return -1;
					return pa - pb;
				}
				case 'rent':
					return a.rent - b.rent;
				case 'size_sqm':
					return a.size_sqm - b.size_sqm;
				case 'reserve_until':
					return a.reserve_until.localeCompare(b.reserve_until);
				case 'move_in':
					return a.move_in.localeCompare(b.move_in);
				case 'floor':
					return a.floor - b.floor;
			}
		});
	}

	let sorted = $derived(sortVacancies(vacancies, $sortField));
</script>

<svelte:window onclick={(e) => { if (showSortMenu) showSortMenu = false; }} />

<div class="mb-4 flex justify-between border-b pb-1">
	<div class="text-sm text-gray-700">
		Sortera efter
		<div class="relative ml-2 inline-block text-left">
			<button
				onclick={(e) => { e.stopPropagation(); showSortMenu = !showSortMenu; }}
				class="inline-flex w-full justify-center gap-x-1.5 rounded-lg px-3 py-2 text-sm font-semibold text-gray-900 hover:bg-gray-100"
			>
				{SORT_NAMES[$sortField]}
				<svg
					class="-mr-1 h-5 w-5 text-gray-400"
					viewBox="0 0 20 20"
					fill="currentColor"
					aria-hidden="true"
				>
					<path
						fill-rule="evenodd"
						d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
						clip-rule="evenodd"
					/>
				</svg>
			</button>

			{#if showSortMenu}
				<div
					class="absolute right-0 z-10 mt-2 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
					role="menu"
				>
					<div class="py-1">
						{#each Object.entries(SORT_NAMES) as [id, name]}
							<button
								role="menuitem"
								onclick={(e) => {
									e.stopPropagation();
									sortField.set(id as SortField);
									showSortMenu = false;
								}}
								class="block w-full px-4 py-2 text-start text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"
							>
								{name}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<div class="-m-2 grid gap-x-2 gap-y-4 sm:grid-cols-2 md:grid-cols-3">
	{#each sorted as property (property.id)}
		<VacancyCard
			{property}
			picture={areas[property.area]?.pictures?.[0]}
		/>
	{/each}
</div>
