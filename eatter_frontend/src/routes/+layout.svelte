<script lang="ts">
    import './styles.css'
    import Button from '$lib/ui_components/Button.svelte'
	import type { PageData } from './$types';
	import { redirect } from '@sveltejs/kit';

    /** @type {import('./$types').PageData} */
    export let data: PageData;

    $: is_logged = data.is_logged;
</script>

<div class="big_container">
    <header>
        <h1> 
            Eatter
        </h1>
    </header>
    
    <div class="little_container">
        <div class="left_container">
            {#if is_logged}
                <a href="/logout">
                    <Button on:click={() => {}}>
                        Wyloguj
                    </Button>
                </a>
            {:else}
                <a href="/login">
                    <Button>
                        Zaloguj
                    </Button>
                </a>
            {/if}
            
            <nav>
                <ul>
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
        </div>
        <main>
            <slot></slot>
        </main>
    </div>
</div>


<style>
    header {
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
    }
</style>