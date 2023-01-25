import type { LayoutServerLoad } from "./$types";

export const load = (async ({locals}) => {
    return {
        auth: locals.auth,
    };
}) satisfies LayoutServerLoad;