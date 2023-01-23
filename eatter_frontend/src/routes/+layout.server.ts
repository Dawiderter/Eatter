import type { LayoutServerLoad } from "./$types";

export const load = (async ({locals}) => {
    return {
        is_logged: locals.is_logged,
    };
}) satisfies LayoutServerLoad;