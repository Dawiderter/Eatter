import { get_session } from "$lib/login";
import type { Handle } from "@sveltejs/kit";

export const handle : Handle = async ({event, resolve}) => {

    event.locals.auth = await get_session(event);

    console.log(event.locals.auth);

    return resolve(event);
};