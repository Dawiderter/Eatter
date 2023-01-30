import { api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    
    const meal = await api_get(fetch, "/meal/" + params.mid);
    const reviews = await api_get(fetch, "/review/meal/" + params.mid);

    return {
        item: meal,
        reviews: reviews,
    }

}) satisfies PageServerLoad;

export const actions = {
    add: async ({fetch, request, params}) => {
        const data = await request.formData();

        const body = data.get('review_body');
        console.log(body);
        const score = data.get('review_score');
        console.log(score);
        const meal_id = params.mid; 
        console.log(meal_id);

        if (meal_id != null && body != null && score != null) {
            await api_post(fetch, "/review/", {
                body: body.valueOf().toString(),
                score: parseInt(score.valueOf().toString()),
                meal_id: parseInt(meal_id)
            });
        }
    }   
} satisfies Actions ;