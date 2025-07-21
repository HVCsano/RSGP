import { apiUrl } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getworkers = await fetch(`${apiUrl}/user/admin/workers/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const workers: {
        name: string;
        access: { address: string; port: string };
        key: string;
    }[] = await getworkers.json();
    return { workers };
}) satisfies PageServerLoad;
