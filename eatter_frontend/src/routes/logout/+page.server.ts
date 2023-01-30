import { api_post } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
    logout: async ({fetch, cookies}) => {
        let res = await api_post(fetch, "/auth/logout", {});
        if (res) {
          cookies.delete("token");
          throw redirect(303, "/")
        }
    }   
} satisfies Actions;