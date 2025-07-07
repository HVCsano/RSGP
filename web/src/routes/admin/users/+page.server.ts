import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getusers = await fetch(`${apiUrl}/user/admin/users`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    if (getusers.ok) {
        const users: {
            username: string;
            password: string;
            permissions: string[];
        }[] = await getusers.json();
        return { users };
    }
    throw redirect(302, "/admin");
}) satisfies PageServerLoad;
