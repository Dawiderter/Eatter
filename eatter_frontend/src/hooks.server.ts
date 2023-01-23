import { get_session } from "$lib/login";
import type { Handle } from "@sveltejs/kit";

export const handle : Handle = async ({event, resolve}) => {

    event.locals.is_logged = await get_session(event);

    return resolve(event);
};