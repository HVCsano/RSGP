import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ parent }) => {
    const p = await parent();
    if (
        !p.layout?.permissions.includes("Admin") &&
        !p.layout?.permissions.includes("AdminPage")
    ) {
        throw redirect(302, "/");
    }
    return {};
}) satisfies LayoutServerLoad;
