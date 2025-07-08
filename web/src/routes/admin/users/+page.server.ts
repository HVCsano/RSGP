import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getusers = await fetch(`${apiUrl}/user/admin/users/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    if (getusers.ok) {
        const users: {
            [key: string]: {
                password: string;
                groups: string[];
            };
        } = await getusers.json();
        return { users };
    }
    throw redirect(302, "/admin");
}) satisfies PageServerLoad;

export const actions = {
    deleteuser: async ({ cookies, request }) => {
        const data = await request.formData();
        const user = data.get("user");
        await fetch(`${apiUrl}/user/admin/users/post`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "modify": "delete",
                "user": user!.toString(),
            },
        });
        throw redirect(302, "/admin/users");
    },
} satisfies Actions;
