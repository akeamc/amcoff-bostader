<script lang="ts">
	import { Temporal } from '@js-temporal/polyfill';
	import type { Property } from '$lib/api';

	let {
		property,
		picture
	}: {
		property: Property;
		picture?: { url: string; alt: string | null };
	} = $props();

	// Urgency check for reservation deadline
	let urgent = $state(false);

	$effect(() => {
		if (!property.reserve_until) return;

		function check() {
			const until = Temporal.PlainDate.from(property.reserve_until).toZonedDateTime({
				timeZone: 'Europe/Stockholm',
				plainTime: Temporal.PlainTime.from('23:59:59.999')
			});
			const seconds = Temporal.Now.instant().until(until.toInstant(), {
				smallestUnit: 'second'
			}).seconds;
			urgent = seconds < 86_400;
		}

		check();
		const interval = setInterval(check, 1000);
		return () => clearInterval(interval);
	});

	// Date formatting helpers
	function formatReserveRange() {
		if (!property.reserve_from || !property.reserve_until) return '';

		const from = Temporal.PlainDate.from(property.reserve_from).toZonedDateTime({
			timeZone: 'Europe/Stockholm',
			plainTime: Temporal.PlainTime.from('00:00:00.000')
		});
		const until = Temporal.PlainDate.from(property.reserve_until).toZonedDateTime({
			timeZone: 'Europe/Stockholm',
			plainTime: Temporal.PlainTime.from('23:59:59.999')
		});

		const sameYear = from.year === until.year;
		const sameYearAndMonth = sameYear && from.month === until.month;
		const currentYear = sameYear && from.year === Temporal.Now.zonedDateTimeISO().year;

		const fromStr = from.toLocaleString('sv', {
			year: sameYear ? undefined : 'numeric',
			month: sameYearAndMonth ? undefined : 'short',
			day: 'numeric'
		});
		const untilStr = until.toLocaleString('sv', {
			year: currentYear ? undefined : 'numeric',
			month: 'short',
			day: 'numeric'
		});
		const separator = sameYearAndMonth ? '–' : ' – ';

		return `Anmälan ${fromStr}${separator}${untilStr}`;
	}

	function formatMoveIn() {
		if (!property.move_in) return '';
		const date = Temporal.PlainDate.from(property.move_in);
		const currentYear = Temporal.Now.zonedDateTimeISO().year;
		return (
			'Tillträde ' +
			date.toLocaleString('sv', {
				year: currentYear === date.year ? undefined : 'numeric',
				month: 'short',
				day: 'numeric'
			})
		);
	}

	let queuePosition = $derived(property.queue_position);
</script>

<a href="/bostad/{property.id}" class="group">
	<article
		class="flex flex-col gap-1 rounded-b-2xl rounded-t-3xl p-2 leading-tight text-neutral-700 group-hover:bg-gray-100 {property.reserved
			? 'bg-pink-100 ring ring-pink-400'
			: ''}"
	>
		<div
			class="relative mb-3 aspect-video w-full overflow-hidden rounded-2xl bg-neutral-100 shadow-sm"
		>
			{#if picture}
				<img
					src={picture.url}
					alt={picture.alt ?? ''}
					class="size-full object-cover transition-transform group-hover:scale-105"
				/>
			{/if}
			<div
				class="absolute right-2 top-2 inline-block rounded-full px-2 py-1 text-sm text-black shadow-md {queuePosition.position === 1
					? 'bg-gradient-to-tr from-yellow-500 to-yellow-400'
					: 'bg-white'}"
				title="Köplats"
			>
				{#if typeof queuePosition.position === 'number'}
					{queuePosition.position} av {queuePosition.total_in_queue}
				{:else}
					{queuePosition.total_in_queue} i kö
				{/if}
			</div>
		</div>
		<h3 class="text-lg font-medium leading-none tracking-tight text-black">{property.area}</h3>
		<div>
			{property.short_description} ⋅
			{property.size_sqm.toLocaleString('sv', {
				minimumFractionDigits: 1,
				maximumFractionDigits: 1
			})} m² ⋅ vån {property.floor}
			{#if 'facing' in property} ⋅ {property.facing}{/if}
		</div>
		<div>
			<span class={urgent ? 'font-medium text-red-500' : ''}>{formatReserveRange()}</span>
			{' ⋅ '}
			{formatMoveIn()}
		</div>
		<div class="font-medium text-black">
			{property.rent.toLocaleString('sv')}&nbsp;kr/mån
		</div>
	</article>
</a>