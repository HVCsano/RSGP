import { apiUrl } from "$lib/api";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async () => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    login: async ({ cookies, request }) => {
        const data = await request.formData();
        const username = data.get("username");
        const password = data.get("password");
        const agent = data.get("agent");
        try {
            const login = await fetch(`${apiUrl}/auth/login`, {
                headers: {
                    username: username!.toString(),
                    password: password!.toString(),
                    agent: agent!.toString(),
                },
            });
            if (login.status === 200) {
                const session = await login.text();
                cookies.set("session", session, {
                    path: "/",
                    httpOnly: true,
                    secure: true,
                    sameSite: "lax",
                });
                return {
                    success: true,
                };
            }
            if (login.status === 401) {
                return {
                    error: "401",
                };
            }
            if (login.status === 406) {
                return {
                    error: "406",
                };
            }
        } catch (e) {
            return {
                error: `unknown/${e}`,
            };
        }
    },
} satisfies Actions;
