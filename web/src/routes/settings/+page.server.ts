import { apiUrl } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const sessions = await fetch(`${apiUrl}/user/sessions/get`, {
        headers: {
            "Authorization": `Bearer ${cookies.get("session")!}`,
        },
    });
    const sess: {
        id: string;
        agent: string;
        login_time: number;
        exp_time: number;
    }[] = await sessions.json();
    return {
        sessions: sess,
    };
}) satisfies PageServerLoad;

export const actions = {
    remove: async ({ cookies, request }) => {
        const data = await request.formData();
        const session = cookies.get("session")!;
        const id = data.get("id")!.toString();
        await fetch(`${apiUrl}/user/sessions/remove`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${session}`,
                "session_id": id,
            },
        });
    },
    remove_all: async ({ cookies }) => {
        const session = cookies.get("session")!;
        await fetch(`${apiUrl}/user/sessions/remove_all`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${session}`,
            },
        });
    },
} satisfies Actions;
