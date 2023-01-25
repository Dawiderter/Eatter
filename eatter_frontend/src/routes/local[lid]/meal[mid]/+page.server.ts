import { fetch_meal, fetch_post } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const meal = await fetch_meal(event, event.params.mid);

    return {
        item: meal
    }

}) satisfies PageServerLoad;