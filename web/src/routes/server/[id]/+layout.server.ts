import { apiUrl } from "$lib/api";
import type { Egg, ServerStates } from "$lib/types";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies, params }) => {
    const f = await fetch(`${apiUrl}/user/server/get`, {
        method: "POST",
        headers: {
            "Authorization": `Bearer ${cookies.get("session")}`,
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            id: params.id,
        }),
    });
    const info: {
        name: string;
        id: string;
        egg: Egg;
        state: ServerStates;
    } = await f.json();
    const log = await fetch(`${apiUrl}/user/server/log`, {
        method: "POST",
        headers: {
            "Authorization": `Bearer ${cookies.get("session")}`,
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            id: params.id,
        }),
    });
    return { info, log: await log.text() };
}) satisfies LayoutServerLoad;
