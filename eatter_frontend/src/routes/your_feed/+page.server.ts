import { fetch_global_feed, fetch_personal_feed } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const posts = await fetch_personal_feed(event);

    console.log(posts.items);

    return {
        items: posts
    }

}) satisfies PageServerLoad;