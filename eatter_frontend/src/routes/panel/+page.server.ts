import { error } from '@sveltejs/kit';
import { api_del, api_get, api_post } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({fetch, params, locals}) => {
    if (locals.auth == null || locals.auth.company_id == null) {
        throw error(404, {
            message: 'Not permitted'
        });
    }

    const my_locals =  await api_get(fetch, "/local/my");
    return {
        locals: my_locals,
    }
}) satisfies PageServerLoad;

// {name: "name", type: "text"},
//         {name: "phone_number", type: "text"},
//         {name: "contact_email", type: "text"},
//         {name: "address", type: "text"}]}>

export const actions = {
    add: async ({fetch, request, locals}) => {
      const data = await request.formData();
  
      const name = data.get('name');
      const phone = data.get('phone_number');
      const email = data.get('contact_email');
      const address = data.get('address');
  
      if (name != null && phone != null && email != null && address != null && locals.auth.company_id != null) {
        let tok = await api_post(fetch, "/local", 
            {
                name : name.valueOf().toString(), 
                phone_num : phone.valueOf().toString(), 
                contact_email : email.valueOf().toString(), 
                address : address.valueOf().toString(),
                local_id: locals.auth.company_id,
            });
      }
    },
    del: async ({fetch, request, locals}) => {
        const data = await request.formData();
    
        const local_id = data.get("local_id");
    
        if (local_id != null) {
          let tok = await api_del(fetch, "/local/" + local_id);
        }
      }  

  } satisfies Actions ;