import { create_session } from "$lib/login";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async (event) => {
    const data = await event.request.formData();

    const email = data.get('email');
    const pass = data.get('pass');

    
    if (email == null || pass == null) {
      return;
    }

    if(await create_session(event, email?.valueOf().toString(), pass?.valueOf().toString())) {
      
    }
  }   
} satisfies Actions ;