import type { PageLoad } from './$types';

export const load = (({ params }) => {
    return {
      lid: params.lid,
    };
}) satisfies PageLoad;