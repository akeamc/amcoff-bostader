<script lang="ts">
	import { login } from '$lib/api';
	import { auth } from '$lib/stores';

	let { onclose }: { onclose: () => void } = $props();

	let email = $state('');
	let password = $state('');
	let invalidCredentials = $state(false);
	let pending = $state(false);

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		invalidCredentials = false;
		pending = true;
		try {
			const res = await login({ email, password });
			if (res === 'invalid-credentials') {
				invalidCredentials = true;
			} else {
				auth.setUser(res);
				onclose();
			}
		} finally {
			pending = false;
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) onclose();
	}
</script>

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="fixed inset-0 z-50 flex items-end justify-center bg-black/15 sm:items-start sm:justify-end sm:p-4"
	onclick={handleBackdropClick}
>
	<div
		class="w-full rounded-t-2xl border bg-white text-sm shadow-lg sm:w-80 sm:rounded-xl"
		role="dialog"
		aria-modal="true"
		aria-label="Logga in"
	>
		<form onsubmit={handleSubmit} class="space-y-4 p-3">
			<p class="text-neutral-700">
				Använd samma inloggningsuppgifter som på afbostader.se.
			</p>

			<div>
				<label for="email" class="text-sm font-medium">E-postadress</label>
				<input
					id="email"
					required
					type="email"
					class="mt-2 block w-full rounded-lg border bg-neutral-50 px-3 py-1.5 text-sm focus:outline-none focus:outline-2 focus:-outline-offset-2 focus:outline-green-500 {invalidCredentials
						? 'border-red-400'
						: ''}"
					bind:value={email}
				/>
			</div>
			<div>
				<label for="password" class="text-sm font-medium">Lösenord</label>
				<input
					id="password"
					required
					type="password"
					class="mt-2 block w-full rounded-lg border bg-neutral-50 px-3 py-1.5 text-sm focus:outline-none focus:outline-2 focus:-outline-offset-2 focus:outline-green-500 {invalidCredentials
						? 'border-red-400'
						: ''}"
					bind:value={password}
				/>
			</div>
			{#if invalidCredentials}
				<p class="rounded-lg border border-red-300 bg-red-50 p-4 text-red-500">
					Felaktigt användarnamn eller lösenord.
				</p>
			{/if}
			<button
				type="submit"
				disabled={pending}
				class="block w-full rounded-lg bg-gradient-to-tr from-green-500 to-green-400 py-2 text-sm font-semibold text-white shadow disabled:opacity-50"
			>
				Logga in
			</button>
		</form>
	</div>
</div>
