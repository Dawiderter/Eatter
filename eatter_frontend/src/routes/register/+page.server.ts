import { api_post } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  register: async ({fetch, request}) => {
    const data = await request.formData();

    const email = data.get('email');
    const pass = data.get('pass');
    const nick = data.get('nick');
    
    if (email != null && pass != null && nick != null) {
      await api_post(fetch, "/auth/register", {email : email.valueOf().toString(), pass : pass.valueOf().toString(), nick: nick.valueOf().toString()});
    }
  }   
} satisfies Actions ;