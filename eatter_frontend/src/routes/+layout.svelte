<script lang="ts">
    import '../app.css'
    import Button from '$lib/ui_components/Button.svelte'
	import type { PageData } from './$types';
	import { redirect } from '@sveltejs/kit';
	import Navbar from '$lib/ui_components/sidebar/Navbar.svelte';
    import Sidebar from '$lib/ui_components/sidebar/Sidebar.svelte';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    let open = false;

    $: is_logged = data.is_logged;
</script>

<svelte:head>
	<link href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css" rel="stylesheet"/>
</svelte:head>

<Sidebar bind:open = {open} bind:is_logged = {is_logged}/>
<Navbar bind:sidebar={open}/>
<div class="flex flex-col items-center justify-center">
    <div class = "flex items-center justify-center h-screen w-4/12">
        <slot></slot>
    </div>
</div>


<style>
    /* header {
        text-align: center;
        padding: 10px 0px;
        margin: 10px;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    }

    main {
        margin: 10px;
        width: 80%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .big_container {
        display: flex;
        flex-direction: column;
    }

    .little_container {
        display: flex;
        flex-direction: row;
    }

    .left_container {
        padding: 10px;
        max-width: 200px;
        width: 20%;
        min-width: 100px;
    }

    ul {
        padding: 0px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        list-style: none;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    }
    
    li {
        margin: 10px 0px;
        padding: 10px;
        text-align: center;
        background-color: var(--color-bg-2);
        border-radius: var(--corner-radius);
        font-weight: bold;
    }

    a, a:hover, a:visited, a:active {
        color: inherit;
        text-decoration: none;
    }

    li:hover {
        background-color: var(--color-fg-1);
    } */
</style>