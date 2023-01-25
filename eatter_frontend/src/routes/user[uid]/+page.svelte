<script lang="ts">
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";
	import User from "$lib/ui_components/User.svelte";
    import type { PageData } from "./$types";

    export let data : PageData;
</script>

<div>
    <User u={data.item}/>
    {#if data.auth.user_id == data.uid}
		<form class="w-4/12 input-group z-0" method="post">
		    Zmień bio:
			<ResizableInput name="bio_body" class = "w-8/12" minRows={2} maxRows={5}/>
			<button>Zamieść</button>
		</form>
	{/if}
    <div>
        <h2>Followers:</h2>
        {#each data.followers as u}
            <User u={u}/>
        {/each}
    </div>
</div>