import { api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    
    const post = await api_get(fetch, "/review/" + params.pid);
    const comments = await api_get(fetch, "/comment/review/" + params.pid);

    return {
        post: post,
        comments: comments
    }

}) satisfies PageServerLoad;

export const actions = {
    add: async ({fetch, request, params}) => {
        const data = await request.formData();

        const body = data.get('comment_body');
        const review_id = params.pid; 

        console.log(body);
        console.log(review_id);
        
        if (review_id != null && body != null) {
            await api_post(fetch, "/comment/", {review_id : parseInt(review_id), body: body.valueOf().toString()});
        }
    }   
} satisfies Actions ;