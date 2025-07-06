import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies, url }) => {
    if (!cookies.get("session") && url.pathname !== "/login") {
        throw redirect(302, "/login");
    }
    return {};
}) satisfies LayoutServerLoad;
