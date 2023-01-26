import { change_bio, fetch_user, fetch_user_followed, fetch_user_followers, follow, unfollow } from '$lib/post';
import type { Actions, PageServerLoad } from './$types';


export const load = ( async (event) => {
    const user = await fetch_user(event, event.params.uid);
    const followers = await fetch_user_followers(event, event.params.uid);
    const followed =  await fetch_user_followed(event, event.params.uid);
    return {
        item: user,
        followers: followers,
        followed: followed,
        uid: event.params.uid,
    }
}) satisfies PageServerLoad;

export const actions = {
    bio: async (event) => {
        const data = await event.request.formData();
        
        const body = data.get('bio_body'); 
        
        if (body != null) {
            await change_bio(event, body.valueOf().toString());
        }
    },
    follow: async (event) => {
        const uid = event.params.uid; 
        
        if (uid != null) {
            await follow(event, parseInt(uid));
        }
    },
    unfollow: async (event) => {
        const uid = event.params.uid; 
        
        if (uid != null) {
            await unfollow(event, parseInt(uid));
        }
    } 
} satisfies Actions ;