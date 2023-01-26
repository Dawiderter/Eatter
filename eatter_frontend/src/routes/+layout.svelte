<script lang="ts">
    import '../app.css'
	import type { PageData } from './$types';
	import { redirect } from '@sveltejs/kit';
	import Navbar from '$lib/ui_components/sidebar/Navbar.svelte';
    import Sidebar from '$lib/ui_components/sidebar/Sidebar.svelte';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    let open = false;

</script>

<svelte:head>
	<link href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css" rel="stylesheet"/>
</svelte:head>

<Sidebar bind:open = {open} auth={data.auth}/>
<Navbar bind:sidebar={open}/>
<div class = "mt-[76px]">
    <slot></slot>
</div>