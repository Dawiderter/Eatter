import type { RequestEvent } from "@sveltejs/kit";

export const get_session = async (event : RequestEvent) => {
    let token = event.cookies.get("token");
    if (token) {
        let auth = await event.fetch("http://0.0.0.0:3000/auth?token=" + token);
        console.log(auth);
        return auth.ok;
    }
    else {
        return false
    }

}

export const drop_session = async (event : RequestEvent) => {
    let token = event.cookies.get("token");
    if (token) {
        let auth = await event.fetch("http://0.0.0.0:3000/logout?token=" + token, {
            method : "DELETE"
        });
        event.cookies.delete("token");
        return auth.ok;
    }
    else {
        return false
    }
    
}

export const create_session = async (event : RequestEvent, email: string, pass: string) => {

    const resp = await event.fetch("http://0.0.0.0:3000/login", {
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify({email : email, pass : pass}),
    });

    if (resp.status == 200) {
        const res = await resp.json();
        event.cookies.set("token", res.token);
    }

}