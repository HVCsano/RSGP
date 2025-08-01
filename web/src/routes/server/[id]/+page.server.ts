import type { Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { apiUrl } from "$lib/api";

export const load = (async () => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    startserver: async ({ cookies, request }) => {
        const data = await request.formData();
        const id = data.get("id") as string;
        await fetch(`${apiUrl}/user/server/run`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ id }),
        });
    },
} satisfies Actions;
