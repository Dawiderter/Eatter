<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    export let data : import("./$types").PageData;
</script>


<section>
    <div class = "flex flex-col items-center justify-center">
        {#each data.items as p}
            <Post
            author = {p.l_id}
            body = {p.r_body}
            score = {p.r_score}
            meal_name = {p.m_name}
            created_at = {p.r_created_at}
            local_name = {p.l_name}>
            </Post>
        {/each}
    </div>
</section>
