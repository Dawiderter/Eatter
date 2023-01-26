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

        const email = data.get('email');
        const pass = data.get('pass');

        
        if (email == null || pass == null) {
            return;
        }
        await post_comment(event, email?.valueOf().toString(), pass?.valueOf().toString());
    }   
} satisfies Actions ;

