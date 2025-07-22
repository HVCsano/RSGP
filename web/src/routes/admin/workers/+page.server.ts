import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getworkers = await fetch(`${apiUrl}/user/admin/workers/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const workers: {
        name: string;
        access: { address: string; port: string };
        key: string;
    }[] = await getworkers.json();
    return { workers };
}) satisfies PageServerLoad;

export const actions = {
    addworker: async ({ request, cookies }) => {
        const data = await request.formData();
        const address = data.get("address") as string;
        const name = data.get("name") as string;
        const port = data.get("port") as string;
        const https = !!data.get("https");
        const check = await fetch(`${apiUrl}/user/admin/workers/check`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                address,
                port: Number(port),
                protocol: https ? "Https" : "Http",
            }),
        });
        if (!check.ok) {
            return {
                addworker: {
                    error: "checkFailed",
                },
            };
        }
        const add = await fetch(`${apiUrl}/user/admin/workers/add`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                address,
                name,
                port: Number(port),
                protocol: https ? "Https" : "Http",
            }),
        });
        if (!add.ok) {
            if (add.status === 402) {
                return {
                    addworker: {
                        error: "already",
                    },
                };
            }
        }
        throw redirect(302, new URL(request.url).pathname);
    },
    delete: async ({ request, cookies }) => {
        const data = await request.formData();
        const worker = data.get("worker") as string;
        await fetch(`${apiUrl}/user/admin/workers/delete`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                worker,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
} satisfies Actions;
