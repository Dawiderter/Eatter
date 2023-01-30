import { api_del, api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params}) => {
    
    let meals = await api_get(fetch, "/meal/local/" + params.lid);

    return {
        items: meals,
    }

}) satisfies PageServerLoad;

export const actions = {
    add: async ({fetch, request, params}) => {
      const data = await request.formData();
  
      const name = data.get('name');
      const price = data.get('price');
  
      if (name != null && price != null) {
        let tok = await api_post(fetch, "/meal", {name : name.valueOf().toString(), price: parseFloat(price.valueOf().toString()), local_id: parseInt(params.lid)});
      }
    },
    del: async ({fetch, request, locals}) => {
        const data = await request.formData();
    
        const local_id = data.get("meal_id");
    
        if (local_id != null) {
          let tok = await api_del(fetch, "/meal/" + local_id);
        }
      }  
  } satisfies Actions ;