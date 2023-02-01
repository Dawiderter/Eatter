export const api_get = async (sfetch : typeof fetch, resource : string) => {
    const resp = await sfetch("http://api" + resource);

    if (resp.status == 200) {
        const item = await resp.json();
        return item;
    }

    return null;
}

export const api_post_ret = async (sfetch : typeof fetch, resource : string, data : any) => {
    const resp = await sfetch("http://api" + resource,{
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify(data),
    });

    if (resp.status == 200) {
        const item = await resp.json();
        return item;
    }

    return null;
}

export const api_post = async (sfetch : typeof fetch, resource : string, data : any) => {
    const resp = await sfetch("http://api" + resource,{
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "POST",
        body: JSON.stringify(data),
    });

    return resp.status == 200;
}

export const api_del = async (sfetch : typeof fetch, resource : string, data : any) => {
    const resp = await sfetch("http://api" + resource,{
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "DELETE",
        body: JSON.stringify(data),
    });

    return resp.status == 200;
}

export const api_patch = async (sfetch : typeof fetch, resource : string, data : any) => {
    const resp = await sfetch("http://api" + resource,{
        headers: new Headers([['Content-Type', 'application/json']]),
        method: "PATCH",
        body: JSON.stringify(data),
    });

    return resp.status == 200;
}