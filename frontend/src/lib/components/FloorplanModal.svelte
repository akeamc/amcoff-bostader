<script lang="ts">
	import { API_URL } from '$lib/api';
	import type { PropertyDetail } from '$lib/api';
	import { formatPostalCode } from '$lib/utils';

	let { property, onclose }: { property: PropertyDetail; onclose: () => void } = $props();

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) onclose();
	}
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="fixed inset-0 z-10 flex items-center justify-center overflow-y-auto bg-black/15 p-4"
	onclick={handleBackdropClick}
>
	<div class="w-full max-w-screen-lg rounded-xl bg-white p-6" role="dialog" aria-modal="true">
		<div class="mb-4">
			<div class="text-lg font-medium">{property.address.street}</div>
			<div class="text-sm text-neutral-700">
				{formatPostalCode(property.address.postal_code)}
				{property.address.city}
			</div>
		</div>
		<img
			src="{API_URL}/vacancies/{property.id}/floorplan"
			alt="Planritning"
			class="mx-auto max-h-[70vh] w-auto"
		/>
	</div>
</div>
