import type { PageLoad } from './$types';

export const load = (({ params }) => {
    return {
      pid: params.pid,
    };
}) satisfies PageLoad;