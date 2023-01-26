<script lang="ts">
    import '../../../app.css';
	import type { Auth } from '$lib/login';
	export let open = false
    export let auth: Auth | null = null;
</script>
 
<aside class="absolute flex items-center justify-center h-screen bg-gray-200 border-r-2 shadow-lg p-12" class:open>
    <nav class="p-12 text-xl font-bold">
        <ul class = "space-y-7">
                {#if auth != null}
                <li>
                    <a href="/logout">Wyloguj</a>
                </li>
                {:else}
                <li>
                    <a href="/login">Zaloguj</a>
                </li>
                {/if}
            <li>
                <a href="/feed">Globalny Feed</a>
            </li>
            {#if auth != null}
                <li>
                    <a href="/">Twój Feed</a>
                </li>
                <li>
                    <a href="/user{auth.user_id}">Profil</a>
                </li>
            {/if}
        </ul>
    </nav>
</aside>

<style lang="postcss">
	aside {
		left: -100%;
		transition: left 0.2s
	}
	
	.open {
		left: 0
	}
</style>