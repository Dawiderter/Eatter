<script>
    import Button from '$lib/ui_components/Button.svelte'
	import { redirect } from '@sveltejs/kit';
	export let open = false
    export let is_logged = false;
</script>
 
<div class = "relative">
    <aside class="absolute w-full flex items-center justify-center h-screen bg-gray-200 border-r-2 shadow-lg p-12" class:open>
        <nav class="p-12 text-xl font-bold">
            <ul class = "space-y-7">
                <li>
                    {#if is_logged}
                    <a href="/logout">Wyloguj</a>
                    {:else}
                    <a href="/login">Zaloguj</a>
                    {/if}
                </li>
                <li>
                    <a href="/feed">Globalny Feed</a>
                </li>
                {#if is_logged}
                    <li>
                        <a href="/">Twój Feed</a>
                    </li>
                {/if}
                <li>
                    <a href="/">Profil</a>
                </li>
            </ul>
        </nav>
    </aside>
</div>

<style lang="postcss">
	aside {
		left: -100%;
		transition: left 0.2s
	}
	
	.open {
		left: 0
	}
</style>