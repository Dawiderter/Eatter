<script lang="ts">
	import Comment from "$lib/ui_components/Comment.svelte";
import Post from "$lib/ui_components/Post.svelte";
	import { each } from "svelte/internal";

    export let data : import("./$types").PageData;

    // const p = data.items[0];

    let comments = [5,2];
</script>
<section>
    <div>
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
    <div>
        <!-- {#each comments as comm}
            <Comment
                author = {author}
                body = {body}
                score = {score}
                meal = {meal}
                comments_num = {comments_num}>
            </Comment>
        {/each} -->
    </div>
</section>


<style>
    div {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
</style>