<script lang="ts">
	import Comment from "$lib/ui_components/Comment.svelte";
import Post from "$lib/ui_components/Post.svelte";
	import { each } from "svelte/internal";

    export let data : import("./$types").PageData;

    const post = data.post;
    const comments = data.comments;
</script>
<section>
    <div>
        <Post
        post_id = {post.r_id}
        meal_id = {post.m_id}
        author_id = {post.l_id}
        author = {post.l_id}
        body = {post.r_body}
        score = {post.r_score}
        meal_name = {post.m_name}
        created_at = {post.r_created_at}
        local_name = {post.l_name}
        check_comments_visible = {false}>
        </Post>
        <h1 class = "mt-[20px] text-xl font-raleway">Comments:</h1>
    </div>
    <div>
        {#each comments as comm}
            <Comment
                author = {comm.u_id}
                body = {comm.c_body}
                created_at = {comm.c_created_at}>
            </Comment>
        {/each}
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