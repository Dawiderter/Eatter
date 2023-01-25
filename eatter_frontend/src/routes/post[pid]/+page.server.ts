import { fetch_post } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const post = await fetch_post(event, event.params.pid);

    return {
        items: post
    }

}) satisfies PageServerLoad;