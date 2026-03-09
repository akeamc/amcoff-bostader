<script lang="ts">
import type { PageData } from './$types';
import type { PropertyDetail, AreaDetail } from '$lib/api';
import AreaPictures from '$lib/components/AreaPictures.svelte';
import FloorplanModal from '$lib/components/FloorplanModal.svelte';
import { getVacancy, getArea } from '$lib/api';
import { onMount } from 'svelte';

let { data }: { data: PageData } = $props();

let clientProperty: PropertyDetail | null = $state(null);
let clientArea: AreaDetail | null = $state(null);
let showFloorplan = $state(false);

let property = $derived(clientProperty ?? data.property);
let area = $derived(clientArea ?? data.area);

onMount(async () => {
if (!data.property) {
const id = parseInt(window.location.pathname.split('/').pop() ?? '0');
clientProperty = await getVacancy(id);
clientArea = await getArea(clientProperty.area).catch(() => null);
}
});
</script>

{#if property}
<main class="mx-auto w-full max-w-screen-lg p-4">
<h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">
{property.short_description}, {property.area}
</h1>
<p class="text-neutral-700">{property.address.street}</p>

<AreaPictures pictures={area?.pictures ?? []} />

<button
onclick={() => (showFloorplan = true)}
class="rounded-md border bg-white px-4 py-2 text-sm font-medium shadow-sm hover:bg-neutral-100 focus:outline-none focus:outline-1 focus:outline-white"
>
Planritning
</button>
</main>

{#if showFloorplan}
<FloorplanModal {property} onclose={() => (showFloorplan = false)} />
{/if}
{/if}
