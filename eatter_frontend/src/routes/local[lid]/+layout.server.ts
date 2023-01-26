import { fetch_local } from "$lib/post";
import type { LayoutServerLoad } from "./$types";

export const load = (async (event) => {
    const local = await fetch_local(event, event.params.lid);
  
    return {
      local: local,
    };
}) satisfies LayoutServerLoad;