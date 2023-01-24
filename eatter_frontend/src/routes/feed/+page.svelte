<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    export let data : import("./$types").PageData;
</script>


<section>
    <div>
        {#each data.items as p}
            <Post
            author = {p.review.author_id}
            body = {p.review.body}
            score = {p.review.score}
            meal = {p.meal.name}
            comments_num = {p.comm_count}>
            </Post>
        {/each}
    </div>
</section>

<style>
    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
    div {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
    input {
        border-radius: var(--corner-radius);
        border-style: none;
        padding: 5px;
        background-color: var(--color-bg-1);
    }
</style>