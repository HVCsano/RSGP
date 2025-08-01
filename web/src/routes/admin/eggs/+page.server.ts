import { apiUrl } from "$lib/api";
import type { Egg } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const geteggs = await fetch(`${apiUrl}/user/admin/eggs/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const eggs: {
        [key: string]: Egg;
    } = await geteggs.json();
    return { eggs };
}) satisfies PageServerLoad;
