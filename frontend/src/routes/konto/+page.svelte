<script lang="ts">
import { auth } from '$lib/stores';
import { logout } from '$lib/api';
import { goto } from '$app/navigation';

$effect(() => {
if ($auth.status === 'unauthenticated') {
goto('/');
}
});

async function handleLogout() {
await logout();
auth.setUnauthenticated();
}
</script>

{#if $auth.status === 'authenticated'}
<main class="mx-auto w-full max-w-screen-lg p-4">
<h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">
Hej {$auth.user.first_name}!
</h1>
<button
onclick={handleLogout}
class="mt-8 rounded-md bg-red-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-red-600"
>
Logga ut
</button>
</main>
{/if}
