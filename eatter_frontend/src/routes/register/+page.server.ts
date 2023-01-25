import { create_session, register } from "$lib/login";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async (event) => {
    const data = await event.request.formData();

    const email = data.get('email');
    const pass = data.get('pass');
    const nick = data.get('nick');
    
    if (email != null && pass != null && nick != null) {
      await register(event, email.valueOf().toString(), pass.valueOf().toString(), nick.valueOf().toString());
    }
  }   
} satisfies Actions ;