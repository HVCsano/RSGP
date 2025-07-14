import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getusers = await fetch(`${apiUrl}/user/admin/users/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    const getgroups = await fetch(`${apiUrl}/user/admin/users/getgroups`, {
        headers: {
            "Authorization": `Bearer ${cookies.get("session")!}`,
        },
    });
    if (getusers.ok) {
        const users: {
            [key: string]: {
                password: string;
                groups: string[];
            };
        } = await getusers.json();
        const groups: string[] = await getgroups.json();
        return { users, groups };
    }
    throw redirect(302, "/admin");
}) satisfies PageServerLoad;

export const actions = {
    deleteuser: async ({ cookies, request }) => {
        const data = await request.formData();
        const user = data.get("user");
        await fetch(`${apiUrl}/user/admin/users/post`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "modify": "delete",
                "user": user!.toString(),
            },
        });
        throw redirect(302, new URL(request.url).pathname);
    },
    changepassword: async ({ cookies, request }) => {
        const data = await request.formData();
        const newpassword = data.get("password");
        const user = data.get("user");
        const clearsessions = !!data.get("clearsessions");
        await fetch(
            `${apiUrl}/user/admin/users/changepassword`,
            {
                method: "POST",
                headers: {
                    "Authorization": `Bearer ${cookies.get("session")}`,
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    user,
                    password: newpassword,
                    clearsessions,
                }),
            },
        );
        throw redirect(302, new URL(request.url).pathname);
    },
    adduser: async ({ cookies, request }) => {
        const data = await request.formData();
        const password = data.get("password");
        const username = data.get("username");
        await fetch(`${apiUrl}/user/admin/users/new`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                password,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
} satisfies Actions;
