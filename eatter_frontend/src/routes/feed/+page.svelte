<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    export let data : import("./$types").PageData;
</script>


<section>
    <div class = "p-5 flex flex-col items-center justify-center space-y-5">
        <h1 class = "mt-[10px] mb-[10px] text-2xl font-raleway">Reviews:</h1>
        {#each data.items as p}
            <Post
            post_id = {p.r_id}
            meal_id = {p.m_id}
            author_id = {p.l_id}
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
