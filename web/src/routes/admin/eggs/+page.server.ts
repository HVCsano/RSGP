import { apiUrl } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const geteggs = await fetch(`${apiUrl}/user/admin/eggs/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const eggs: {
        [key: string]: {
            install: {
                dependency_installs: string[][];
                egg_installs: string[][];
            };
            running: {
                start_command: string;
                running_text: string;
            };
            version: string;
            upstream?: string;
        };
    } = await geteggs.json();
    return { eggs };
}) satisfies PageServerLoad;
