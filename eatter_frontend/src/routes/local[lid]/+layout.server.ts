import { api_get } from "$lib/api";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    let local = await api_get(fetch, "/local/" + params.lid);
    
    return {
      local: local,
    };
}) satisfies LayoutServerLoad;