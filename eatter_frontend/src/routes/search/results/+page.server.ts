import { api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, url}) => {
    const urlParams = url.searchParams.get('tags');

    console.log(urlParams);
    
    const meals = await api_get(fetch, "/search" + "?tags=" + urlParams);

    return {
        meals: meals
    }

}) satisfies PageServerLoad;