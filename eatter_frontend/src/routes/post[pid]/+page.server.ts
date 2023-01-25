import { fetch_post, fetch_comments } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const post = await fetch_post(event, event.params.pid);
    const comments = await fetch_comments(event, event.params.pid);

    return {
        post: post,
        comments: comments
    }

}) satisfies PageServerLoad;