<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    export let data : import("./$types").PageData;
</script>


<section>
    <div class = "p-5 flex flex-col items-center justify-center space-y-5">
        <h1 class = "mt-[10px] mb-[10px] text-2xl font-raleway">Reviews:</h1>
        {#each data.items as p}
            <Post p={p} />
        {/each}
    </div>
</section>
