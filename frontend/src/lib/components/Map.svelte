<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { API_URL, geocode, type Property } from '$lib/api';

	let { vacancies }: { vacancies: Property[] } = $props();

	let mapEl: HTMLDivElement;
	let L: typeof import('leaflet');
	let map: import('leaflet').Map | null = null;
	let markers: import('leaflet').Marker[] = [];

	onMount(async () => {
		L = (await import('leaflet')).default;
		await import('leaflet/dist/leaflet.css');

		map = L.map(mapEl, {
			center: [55.704261, 13.1915074],
			zoom: 14,
			scrollWheelZoom: true
		});

		L.tileLayer('https://tiles.stadiamaps.com/tiles/alidade_smooth/{z}/{x}/{y}.png', {
			attribution:
				'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
		}).addTo(map);

		await loadMarkers(vacancies);
	});

	async function loadMarkers(items: Property[]) {
		if (!map || !L) return;

		// Remove old markers
		markers.forEach((m) => m.remove());
		markers = [];

		const icon = L.divIcon({
			className: 'bg-green-500 rounded-full border-2 border-white size-8 -mt-4 -ml-4',
			iconSize: null as unknown as [number, number]
		});

		// Load each vacancy location
		await Promise.allSettled(
			items.map(async (property) => {
				try {
					const places = await geocode(property.address);
					const place = places?.[0];
					if (!place || !map) return;

					const marker = L.marker([place.lat, place.lon], { icon });
					marker
						.bindPopup(`<strong>${property.area}:</strong> ${property.description}`)
						.addTo(map);
					markers.push(marker);
				} catch {
					// ignore geocoding errors
				}
			})
		);
	}

	$effect(() => {
		if (map && L) {
			loadMarkers(vacancies);
		}
	});

	onDestroy(() => {
		if (map) {
			map.remove();
			map = null;
		}
	});
</script>

<div bind:this={mapEl} class="h-full w-full"></div>
