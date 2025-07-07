import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    await cookies.delete("session", { path: "/" });
    throw redirect(302, "/login");
}) satisfies PageServerLoad;
