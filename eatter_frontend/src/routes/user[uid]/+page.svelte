<script lang="ts">
	import User from "$lib/ui_components/User.svelte";
    import type { PageData } from "./$types";

    export let data : PageData;
</script>

<div>
    <User u={data.item}/>
    <div>
        <h2>Followers:</h2>
        {#each data.followers as u}
            <User u={u}/>
        {/each}
    </div>
</div>