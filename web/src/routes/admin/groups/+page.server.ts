import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

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
