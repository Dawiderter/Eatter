<script lang="ts">
	import Comment from "$lib/ui_components/Comment.svelte";
    import Post from "$lib/ui_components/Post.svelte";
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";

    export let data : import("./$types").PageData;

    const post = data.post;
    const comments = data.comments;
</script>
<section>
    <div>
        <Post
        p = {post}
        comments = {false}>
        </Post>
    </div>
    <div class="space-y-5">
        {#if data.auth}
            <form class = "w-4/12 input-group z-0" action="?/add" method="post">
                <label for = "commentinput">Insert comment</label>
                <ResizableInput
                    class = "w-8/12"
                    name = "comment_body"
                    minRows={2}
                    maxRows={5}
                />
                <button class = "mt-[10px] bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">submit</button>
            </form>
        {/if}
        <h1 class = "mt-[10px] mb-[10px] text-xl font-raleway">Comments:</h1>
        {#each comments as comm}
            <Comment c = {comm}/>
        {/each}
    </div>
</section>


<style>
    div,form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
</style>