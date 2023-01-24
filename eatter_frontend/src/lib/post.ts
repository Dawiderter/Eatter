import type { RequestEvent } from "@sveltejs/kit";

export const fetch_post = async (event : RequestEvent, id: number) => {

    const resp = await event.fetch("http://0.0.0.0:3000/post"+id, {
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify({id: id}),
    });

    if (resp.status == 200) {
        const res = await resp.json();
        event.cookies.set("token", res.token);
    }

}

export const fetch_comments = async (event: RequestEvent) => {

}

export const fetch_all_posts = async (event: RequestEvent) => {
    
}