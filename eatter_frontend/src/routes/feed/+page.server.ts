import { fetch_global_feed } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const posts = await fetch_global_feed(event);

    console.log(posts);

    return {
        items: posts
    }

}) satisfies PageServerLoad;