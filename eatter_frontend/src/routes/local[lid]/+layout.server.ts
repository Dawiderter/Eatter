import { api_get, api_post } from "$lib/api";
import type { Actions, LayoutServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    let local = await api_get(fetch, "/local/" + params.lid);
    
    return {
      local: local,
    };
}) satisfies LayoutServerLoad;