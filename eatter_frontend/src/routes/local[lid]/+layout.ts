import type { LayoutLoad } from "./$types";

export const load = (({ params }) => {
    return {
      lid: params.lid,
    };
}) satisfies LayoutLoad;