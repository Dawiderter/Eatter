import type { RequestEvent } from "@sveltejs/kit";

export const fetch_meals = async (event : RequestEvent, tags : string[]) => {

    const resp = await event.fetch("http://0.0.0.0:3000/grab/meals", {
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify({tag : tags[0]}),
    });

    if (resp.status == 200) {
        const res = await resp.json();
        return res;
    }
    else {
        return null;
    }

}