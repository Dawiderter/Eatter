import { api_get } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({fetch}) => {
    
    const reviews = await api_get(fetch, "/review/all");

    return {
        items: reviews
    }

}) satisfies PageServerLoad;