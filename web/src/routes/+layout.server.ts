import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { apiUrl, type Permissions } from "$lib/api";

export const load = (async ({ cookies, url }) => {
    if (url.pathname !== "/login") {
        if (!cookies.get("session")) {
            throw redirect(302, "/login");
        }
        const ses = cookies.get("session") as string;
        const checkLogin = await fetch(`${apiUrl}/user`, {
            headers: { "Authorization": `Bearer ${ses}` },
        });
        if (checkLogin.status === 200) {
            const json: {
                username: string;
                permissions: Permissions[];
                warnings: { title: string; description: string }[];
            } = await checkLogin.json();
            return {
                layout: json,
            };
        }
        throw redirect(302, "/login");
    }
    return {};
}) satisfies LayoutServerLoad;
