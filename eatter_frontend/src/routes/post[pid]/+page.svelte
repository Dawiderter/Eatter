<script lang="ts">
	import Comment from "$lib/ui_components/Comment.svelte";
import Post from "$lib/ui_components/Post.svelte";
	import { each } from "svelte/internal";

    export let data : import("./$types").PageData;

    let author : string;
    let body : string;
    let meal : string;
    let score : number;
    let comments_num : number;

    let comments = [5,2];
</script>
<section>
    <div>
        <Post
            author = {author}
            body = {body}
            score = {score}
            meal = {meal}
            comments_num = {comments_num}>
        </Post>
    </div>
    <div>
        {#each comments as comm}
            <Comment
                author = {author}
                body = {body}
                score = {score}
                meal = {meal}
                comments_num = {comments_num}>
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