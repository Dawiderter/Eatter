import { change_bio, fetch_user, fetch_user_followers } from '$lib/post';
import type { Actions, PageServerLoad } from './$types';


export const load = ( async (event) => {
    const user = await fetch_user(event, event.params.uid);
    const followers = await fetch_user_followers(event, event.params.uid);
    return {
        item: user,
        followers: followers,
        uid: event.params.uid,
    }
}) satisfies PageServerLoad;

export const actions = {
    default: async (event) => {
        const data = await event.request.formData();

        const body = data.get('bio_body'); 
        
        if (body != null) {
            await change_bio(event, body.valueOf().toString());
        }
    }   
} satisfies Actions ;