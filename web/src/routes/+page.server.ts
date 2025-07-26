import { apiUrl } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getsrvs = await fetch(`${apiUrl}/user/servers/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const servers: {
        [key: string]: {
            name: string;
            owner: string;
            egg: string;
            state: string;
        };
    } = await getsrvs.json();
    return { servers };
}) satisfies PageServerLoad;
