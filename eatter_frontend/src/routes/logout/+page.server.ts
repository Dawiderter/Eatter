import { drop_session } from "$lib/login";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
    default: async (event) => {
      await drop_session(event);
    }   
} satisfies Actions;