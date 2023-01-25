import { fetch_meal, fetch_post, fetch_post_of_meal, post_review } from "$lib/post";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async (event) => {
    
    const meal = await fetch_meal(event, event.params.mid);
    const reviews = await fetch_post_of_meal(event, event.params.mid);

    return {
        item: meal,
        reviews: reviews,
    }

}) satisfies PageServerLoad;

export const actions = {
    default: async (event) => {
        const data = await event.request.formData();

        const body = data.get('review_body');
        console.log(body);
        const score = data.get('review_score');
        console.log(score);
        const meal_id = event.params.mid; 
        console.log(meal_id);

        if (meal_id != null && body != null && score != null) {
            await post_review(event, parseInt(meal_id), body.valueOf().toString(), parseInt(score.valueOf().toString()));
        }
    }   
} satisfies Actions ;