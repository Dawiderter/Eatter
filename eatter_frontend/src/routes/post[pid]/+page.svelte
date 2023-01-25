<script lang="ts">
	import { post_comment } from "$lib/post";
	import Comment from "$lib/ui_components/Comment.svelte";
    import Post from "$lib/ui_components/Post.svelte";
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";

    export let data : import("./$types").PageData;

    let comment_input_body = "";

    let custom_class = "";
	export { custom_class as class };

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
    </div>
    <div>
        <div class = "w-4/12 input-group">
            <label for = "commentinput">Insert comment</label>
            <ResizableInput
                class = "w-8/12"
                bind:value={comment_input_body}  
                minRows={2}
                maxRows={5}
            />
            <button class = "mt-[10px] bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">submit</button>
        </div>
        <h1 class = "mt-[10px] mb-[10px] text-xl font-raleway">Comments:</h1>
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