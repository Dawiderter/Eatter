import { error } from '@sveltejs/kit';
import { api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params, locals}) => {
    if (locals.auth == null || locals.auth.company_id == null) {
        throw error(404, {
            message: 'Not permitted'
        });
    }

    const my_locals =  await api_get(fetch, "/local/my");
    return {
        locals: my_locals,
    }
}) satisfies PageServerLoad;