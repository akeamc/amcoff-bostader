<script lang="ts">
import VacancyGrid from '$lib/components/VacancyGrid.svelte';
import type { PageData } from './$types';
import type { AreaDetail, Property } from '$lib/api';
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { listVacancies, getArea } from '$lib/api';

let { data }: { data: PageData } = $props();

let vacancies: Property[] = $state([]);
let areas: Record<string, AreaDetail> = $state({});
let showMap = $state(false);

$effect(() => {
vacancies = data.vacancies;
areas = data.areas;
});

// Auto-refresh vacancies every 30 seconds
onMount(() => {
const interval = setInterval(async () => {
const fresh = await listVacancies();
vacancies = fresh;
// Update areas for any new areas
const newAreaNames = Array.from(new Set(fresh.map((v) => v.area))).filter(
(name) => !areas[name]
);
if (newAreaNames.length > 0) {
const results = await Promise.allSettled(newAreaNames.map((name) => getArea(name)));
const newAreas = { ...areas };
newAreaNames.forEach((name, i) => {
const result = results[i];
if (result.status === 'fulfilled') newAreas[name] = result.value;
});
areas = newAreas;
}
}, 30_000);
return () => clearInterval(interval);
});

// Dynamically import the map component
let LazyMap = $state<typeof import('$lib/components/Map.svelte').default | null>(null);

$effect(() => {
if (showMap && browser && !LazyMap) {
import('$lib/components/Map.svelte').then((mod) => {
LazyMap = mod.default;
});
}
});
</script>

<main class="mx-auto w-full max-w-screen-xl px-4">
<VacancyGrid {vacancies} {areas} />

<div class="mt-8">
{#if !showMap}
<button
onclick={() => (showMap = true)}
class="rounded-md border bg-white px-4 py-2 text-sm font-medium shadow-sm hover:bg-neutral-50"
>
Visa karta
</button>
{:else}
<div class="h-[500px] w-full overflow-hidden rounded-xl border shadow-sm">
{#if LazyMap}
<LazyMap {vacancies} />
{:else}
<div class="flex h-full items-center justify-center bg-neutral-100 text-neutral-500">
Laddar karta…
</div>
{/if}
</div>
{/if}
</div>
</main>
