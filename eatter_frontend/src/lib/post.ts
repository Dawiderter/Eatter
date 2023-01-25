import type { RequestEvent } from "@sveltejs/kit";

export const fetch_global_feed = async (event : RequestEvent) => {

    const resp = await event.fetch("http://0.0.0.0:3000/grab/feed/global");

    if (resp.status == 200) {
        const res = await resp.json();
        return res;
    }
    else {
        return null;
    }

}

export const fetch_meal = async (event : RequestEvent, meal_id : string) => {
    
    const resp = await event.fetch("http://0.0.0.0:3000/grab/meal/" + meal_id);
    
    if (resp.status == 200) {
        const res = await resp.json();
        console.log(res);
        return res;
    }
    else {
        return null;
    }
}

export const fetch_local_meals = async (event : RequestEvent, local_id : string) => {
    
    const resp = await event.fetch("http://0.0.0.0:3000/grab/local/" + local_id + "/meals");

    
    if (resp.status == 200) {
        const res = await resp.json();
        console.log(res);
        return res;
    }
    else {
        return null;
    }
}

export const fetch_post = async (event : RequestEvent, review_id : string) => {
    
    console.log("Fetching post from: " + "http://0.0.0.0:3000/grab/review/" + review_id);
    const resp = await event.fetch("http://0.0.0.0:3000/grab/review/" + review_id);

    
    if (resp.status == 200) {
        const res = await resp.json();
        console.log(res);
        return res;
    }
    else {
        return null;
    }
}