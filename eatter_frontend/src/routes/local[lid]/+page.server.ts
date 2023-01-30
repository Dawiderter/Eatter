import { api_get } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    
    let meals = await api_get(fetch, "/meal/local/" + params.lid);

    return {
        items: meals,
    }

}) satisfies PageServerLoad;