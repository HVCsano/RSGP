import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    if (!cookies.get("session")) {
        return {
            noauth: true,
        };
    }
    return {};
}) satisfies LayoutServerLoad;
