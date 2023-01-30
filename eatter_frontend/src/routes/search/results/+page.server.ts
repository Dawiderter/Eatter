import { api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, url}) => {
    let params = url.search;
    console.log(params);
    
    const meals = await api_get(fetch, "/search" + params);

    return {
        meals: meals
    }

}) satisfies PageServerLoad;