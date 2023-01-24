<script>
	import Button from "$lib/ui_components/Button.svelte";
	import Post from "$lib/ui_components/Post.svelte";

    const bod = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer aliquet dignissim metus, eget sollicitudin nunc cursus et. Suspendisse potenti. Donec ac interdum elit. Ut ac faucibus nisl. Nam ex dolor, porttitor sit amet commodo finibus, commodo tempor arcu. Donec euismod, leo id consequat libero. "
    const posts = [1,2,3,4,5,6,7,8,9];
</script>


<section>
    <div>
        {#each posts as p}
            <Post
            author = "author1"
            body = {bod}
            score = {5}
            meal = "spaghetti bolognese"
            comments_num = {0}>
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