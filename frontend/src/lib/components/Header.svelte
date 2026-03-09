<script lang="ts">
	import { page } from '$app/stores';
	import { auth } from '$lib/stores';
	import LoginModal from './LoginModal.svelte';

	let showLoginModal = $state(false);
</script>

<header class="mx-auto flex w-full max-w-screen-xl items-center p-4">
	<div class="flex flex-1 justify-center">
		<a href="/" class="mr-auto font-serif text-xl font-medium">
			A<span class="text-neutral-400">(mcof)</span>F
			<span class="max-sm:hidden">Bostäder</span>
		</a>
	</div>
	<div class="flex flex-1 justify-center">
		<nav class="flex items-center rounded-full border px-3 shadow-sm">
			<a
				href="/"
				class="px-3 py-2 text-sm font-medium {$page.url.pathname === '/'
					? 'text-black'
					: 'text-neutral-500 hover:text-black'}"
			>
				Lediga bostäder
			</a>
		</nav>
	</div>
	<div class="flex flex-1 justify-center">
		<div class="ml-auto">
			{#if $auth.status === 'loading'}
				<!-- nothing while loading -->
			{:else if $auth.status === 'authenticated'}
				<a
					href="/konto"
					class="text-md flex size-8 items-center justify-center rounded-full bg-green-500 leading-none text-white"
				>
					{$auth.user.first_name.substring(0, 1)}{$auth.user.last_name.substring(0, 1)}
				</a>
			{:else}
				<button
					onclick={() => (showLoginModal = true)}
					class="group rounded-full border p-1 text-sm font-semibold shadow-sm"
				>
					<div
						class="rounded-full bg-gradient-to-tr from-green-500 to-green-400 px-4 py-1 text-white transition-all group-hover:from-green-600 group-hover:to-green-500"
					>
						Logga in
					</div>
				</button>
			{/if}
		</div>
	</div>
</header>

{#if showLoginModal}
	<LoginModal onclose={() => (showLoginModal = false)} />
{/if}
