<script lang="ts">
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";
	import User from "$lib/ui_components/User.svelte";
    import type { PageData } from "./$types";

    export let data : PageData;
</script>

<div class = "p-[10px] flex flex-col items-center justify-center space-y-5">
    <User u={data.item}/>
    {#if data.auth.user_id == data.uid}
		<form class="w-4/12 input-group z-0" method="post" action="?/bio">
		    <label for = "bio_body" class = "font-raleway">Change bio:</label>
			<ResizableInput name="bio_body" class = "w-8/12" minRows={2} maxRows={5}/>
			<button>confirm</button>
		</form>
	{:else}
        {#if data.followers.some(u => u.u_id === data.auth.user_id)}
            <form class="w-4/12 input-group z-0" method="post" action="?/unfollow">
                <button>Unfollow</button>
            </form>    
        {:else}
            <form class="w-4/12 input-group z-0" method="post" action="?/follow">
                <button>Follow</button>
            </form>
        {/if}
    {/if}
    <div>
        <h2 class = "mt-[10px] mb-[10px] text-2xl font-raleway">Followers:</h2>
        {#each data.followers as u}
            u.u_nick
        {/each}
        <h2 class = "mt-[10px] mb-[10px] text-2xl font-raleway">Followed:</h2>
        {#each data.followed as u}
            u.u_nick
        {/each}
    </div>
</div>