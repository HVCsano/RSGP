import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getservers = await fetch(`${apiUrl}/user/admin/servers/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const servers: {
        [key: string]: {
            name: string;
            owner: string;
            egg: string;
        };
    } = await getservers.json();
    return { servers };
}) satisfies PageServerLoad;

export const actions = {
    addserver: async ({ request, cookies }) => {
        const data = await request.formData();
        const owner = data.get("owner") as string;
        const name = data.get("name") as string;
        const egg = data.get("egg") as string;
        const worker = data.get("worker") as string;
        await fetch(`${apiUrl}/user/admin/servers/add`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                owner,
                name,
                egg,
                worker,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
};
