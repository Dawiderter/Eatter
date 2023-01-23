import { redirect } from "@sveltejs/kit";
import type { Actions } from "./$types";

export const actions = {
    default: async (event) => {
      console.log("pa");
    }   
} satisfies Actions;