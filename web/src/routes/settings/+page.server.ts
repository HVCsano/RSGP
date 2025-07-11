import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
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
    changename: async ({ cookies, request }) => {
        const data = await request.formData();
        const session = cookies.get("session")!;
        const id = data.get("id")!.toString();
        const name = data.get("name")!.toString();
        await fetch(`${apiUrl}/user/sessions/changename`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${session}`,
                "session_id": id,
                "name": encodeURI(name),
            },
        });
        throw redirect(302, new URL(request.url).pathname);
    },
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
        throw redirect(302, new URL(request.url).pathname);
    },
    remove_all: async ({ cookies, request }) => {
        const session = cookies.get("session")!;
        await fetch(`${apiUrl}/user/sessions/remove_all`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${session}`,
            },
        });
        throw redirect(302, new URL(request.url).pathname);
    },
} satisfies Actions;
