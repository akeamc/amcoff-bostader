<script lang="ts">
	import type { Picture } from '$lib/api';

	let { pictures = [] }: { pictures: Picture[] } = $props();
</script>

<div class="my-4 grid grid-cols-4 grid-rows-2 gap-2 sm:gap-3">
	{#if pictures[0]}
		<img
			src={pictures[0].url}
			alt={pictures[0].alt ?? ''}
			class="col-span-2 row-span-2 aspect-square size-full rounded-md object-cover sm:rounded-xl"
		/>
	{:else}
		<div class="col-span-2 row-span-2 aspect-square rounded-md bg-neutral-100 sm:rounded-xl"></div>
	{/if}

	{#each [1, 2, 3, 4] as i}
		{#if pictures[i]}
			<img
				src={pictures[i].url}
				alt={pictures[i].alt ?? ''}
				class="col-span-1 row-span-1 aspect-square size-full rounded-md object-cover sm:rounded-xl"
			/>
		{:else}
			<div class="col-span-1 row-span-1 aspect-square rounded-md bg-neutral-100 sm:rounded-xl"></div>
		{/if}
	{/each}
</div>
