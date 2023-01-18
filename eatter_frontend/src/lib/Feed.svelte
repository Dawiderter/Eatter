<script>
    import Post from "./Post.svelte";
    let posts = [
        "post1",
        "post2",
        "post3"
    ]
</script>

<section>
    {#each posts as post}
    <Post content = {post}/>
    {/each}
</section>

<style>
    section {
        width: auto;
    }
</style>

    