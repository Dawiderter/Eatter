import { api_del, api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    
    const meal = await api_get(fetch, "/meal/" + params.mid);
    const tags = await api_get(fetch, "/meal/tags/" + params.mid);
    const reviews = await api_get(fetch, "/review/meal/" + params.mid);

    return {
        item: meal,
        reviews: reviews,
        tags: tags,
    }

}) satisfies PageServerLoad;

export const actions = {
    add: async ({fetch, request, params}) => {
        const data = await request.formData();

        const body = data.get('review_body');
        console.log(body);
        const score = data.get('rate');
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
    },
    add_tag: async ({fetch, request, params}) => {
        const data = await request.formData();

        const tag_name = data.get('tag');
        const meal_id = params.mid;

        if (meal_id != null && tag_name != null) {
            await api_post(fetch, "/meal/tag", {
                tag_name: tag_name,
                meal_id: parseInt(meal_id)
            });
        }
    },
    del_tag: async ({fetch, request, params}) => {
        const data = await request.formData();

        const tag_name = data.get('tag');
        const meal_id = params.mid;

        if (meal_id != null && tag_name != null) {
            await api_del(fetch, "/meal/tag", {
                tag_name: tag_name,
                meal_id: parseInt(meal_id)
            });
        }
    },
} satisfies Actions ;