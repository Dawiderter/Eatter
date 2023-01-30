import { api_get, api_post } from '$lib/api';
import type { Actions, PageServerLoad } from './$types';


export const load = ( async ({fetch, params}) => {
    const user = await api_get(fetch, "/user/" + params.uid);
    const followers = await api_get(fetch, "/user/followers/" + params.uid);
    const followed =  await api_get(fetch, "/user/followed/" + params.uid);
    return {
        item: user,
        followers: followers,
        followed: followed,
        uid: params.uid,
    }
}) satisfies PageServerLoad;

export const actions = {
    bio: async ({fetch, request}) => {
        const data = await request.formData();
        
        const body = data.get('bio_body'); 
        
        if (body != null) {
            await api_post(fetch, "/user/bio",  { bio : body.valueOf().toString()});
        }
    },
    follow: async ({fetch, params}) => {
        const uid = params.uid; 
        
        if (uid != null) {
            await api_post(fetch, "/user/follow", { follow : parseInt(uid)});
        }
    },
    unfollow: async ({fetch, params}) => {
        const uid = params.uid; 
        
        if (uid != null) {
            await api_post(fetch, "/user/unfollow", { follow : parseInt(uid)});
        }
    } 
} satisfies Actions ;