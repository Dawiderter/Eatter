<script lang="ts">
    import '../../../app.css';
	import type { Auth } from '$lib/login';
	export let open = false
    export let auth: Auth | null = null;
</script>
 
<aside class="absolute flex items-center justify-center h-screen bg-gray-200 border-r-2 shadow-lg p-12" class:open>
    <nav class="p-12 text-xl font-bold">
        <form class = "mb-20 p-15">   
            <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
            <div class="relative">
                <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                    <svg aria-hidden="true" class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
                </div>
                <input type="search" id="default-search" class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500" placeholder="Search meals..." required>
                <button type="submit" class="text-white absolute right-1.5 bottom-2.5 bg-gray-600 hover:bg-gray-700 focus:ring-4 focus:outline-none font-xs rounded-lg text-sm px-1 py-2 ">Search</button>
            </div>
        </form>
        <ul class = "space-y-7">
            {#if auth != null}
            <li>
                <a href="/logout">Log out</a>
            </li>
            {:else}
            <li>
                <a href="/login">Log in</a>
            </li>
            {/if}
            <li>
                <a href="/feed">Global Feed</a>
            </li>
            {#if auth != null}
                <li>
                    <a href="/your_feed">Personal Feed</a>
                </li>
                <li>
                    <a href="/user{auth.user_id}">Profile</a>
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
