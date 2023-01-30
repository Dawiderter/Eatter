import { api_get } from "$lib/api";
import type { Handle, HandleFetch } from "@sveltejs/kit";

export const handle : Handle = async ({event, resolve}) => {

    event.locals.auth = await api_get(event.fetch, "/auth/check");

    console.log(event.locals.auth);

    return resolve(event);
};

export const handleFetch : HandleFetch = async ({event, request}) => {
    if (request.url.startsWith("http://api")) {
        request = new Request(
            request.url.replace("http://api", "http://0.0.0.0:3000"),
            request
        )
        let cookies = event.request.headers.get('cookie');
        if (cookies) {
            request.headers.set('cookie', cookies);
        }
    }
    //console.log(request);
    return fetch(request);
}