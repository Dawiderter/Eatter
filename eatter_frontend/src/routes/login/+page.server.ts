import { api_post, api_post_ret } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  login: async ({fetch, request, cookies}) => {
    const data = await request.formData();

    const email = data.get('email');
    const pass = data.get('pass');

    if (email != null && pass != null) {
      let tok = await api_post_ret(fetch, "/auth/login", {email : email.valueOf().toString(), pass: pass.valueOf().toString()});
      cookies.set("token", tok.token);
      throw redirect(303, "/");
    }
  }   
} satisfies Actions ;