import { create_session } from "$lib/login";
import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
  default: async (event) => {
    console.log("em");
    create_session(event, "em", "pa");
  }   
} satisfies Actions ;