import { fetch_post, fetch_comments, post_comment } from "$lib/post";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const post = await fetch_post(event, event.params.pid);
    const comments = await fetch_comments(event, event.params.pid);

    return {
        post: post,
        comments: comments
    }

}) satisfies PageServerLoad;

export const actions = {
    default: async (event) => {
        const data = await event.request.formData();

        const body = data.get('comment_body');
        const review_id = event.params.pid; 

        console.log(body);
        console.log(review_id);
        
        if (review_id != null && body != null) {
            await post_comment(event, parseInt(review_id), body.valueOf().toString());
        }
    }   
} satisfies Actions ;