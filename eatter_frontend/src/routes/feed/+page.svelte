<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    export let data : import("./$types").PageData;
</script>


<section>
    <div class = "overflow-y-auto h-screen">
        {#each posts as p}
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
