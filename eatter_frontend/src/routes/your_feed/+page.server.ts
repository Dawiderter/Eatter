import { api_get } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({fetch}) => {
    
    const posts = await api_get(fetch, "/review/followed");

    return {
        items: posts
    }

}) satisfies PageServerLoad;