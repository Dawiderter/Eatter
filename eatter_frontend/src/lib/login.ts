import type { RequestEvent } from "@sveltejs/kit";

export const get_session = async (event : RequestEvent) => {
    let token = event.cookies.get("token");
    let auth = await event.fetch("http://0.0.0.0:3000/auth/" + token);

    return auth.ok;
}

export const create_session = async (event : RequestEvent, email: string, pass: string) => {

    let auth = await event.fetch("http://0.0.0.0:3000/login", {
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify({email : email, pass : pass}),
    }).then((response) => response.json())
    .then((tok) => event.cookies.set("token", tok.token))

    console.log(auth);
}