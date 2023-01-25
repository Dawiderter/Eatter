import { fetch_local_meals, fetch_meal, fetch_post } from "$lib/post";
import type { PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const meals = await fetch_local_meals(event, event.params.lid);

    return {
        items: meals
    }

}) satisfies PageServerLoad;