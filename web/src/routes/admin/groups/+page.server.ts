import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getgroups = await fetch(`${apiUrl}/user/admin/groups/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    if (getgroups.ok) {
        const groups: { [key: string]: string[] } = await getgroups.json();
        return { groups };
    }
    throw redirect(302, "/admin");
}) satisfies PageServerLoad;

export const actions = {
    addgroup: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get("name");
        await fetch(`${apiUrl}/user/admin/groups/add`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
    deletegroup: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get("name");
        await fetch(`${apiUrl}/user/admin/groups/remove`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
} satisfies Actions;
