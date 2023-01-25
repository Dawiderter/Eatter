import { fetch_user, fetch_user_followers } from '$lib/post';
import type { PageServerLoad } from './$types';


export const load = ( async (event) => {
    const user = await fetch_user(event, event.params.uid);
    const followers = await fetch_user_followers(event, event.params.uid);
    return {
        item: user,
        followers: followers,
    }
}) satisfies PageServerLoad;