import type { PageLoad } from './$types';

export const load = (({ params }) => {
    return {
      mid: params.mid,
    };
}) satisfies PageLoad;