<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { auth } from '$lib/stores';
	import { getUser } from '$lib/api';
	import { onMount } from 'svelte';

	let { children } = $props();

	onMount(async () => {
		const user = await getUser();
		if (user === 'unauthenticated') {
			auth.setUnauthenticated();
		} else {
			auth.setUser(user);
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>A(mcof)F Bostäder</title>
</svelte:head>

<div class="flex min-h-screen flex-col">
	<Header />
	{@render children()}
	<Footer />
</div>

