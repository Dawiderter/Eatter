import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';


export const load = (({ params }) => {
    return {
      title: params.name,
      content: 'Welcome to our blog. Lorem ipsum dolor sit amet...'
    };
}) satisfies PageLoad;