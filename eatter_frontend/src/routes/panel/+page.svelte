<script lang="ts">
    import { page } from '$app/stores';
	import Local from '$lib/ui_components/Local.svelte';

    import type { PageData } from "./$types";

    export let data : PageData;
</script>

<div class = "flex flex-col items-center justify-center">
    <h1 class = "mt-[10px] text-2xl font-raleway">Manage your company</h1>
    {#each data.locals as local}
        <Local l = {local}></Local>
    {/each}
</div>
