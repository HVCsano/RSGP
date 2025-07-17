import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";
import type { Actions, PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    const getgroups = await fetch(`${apiUrl}/user/admin/groups/get`, {
        headers: { "Authorization": `Bearer ${cookies.get("session")}` },
    });
    if (getgroups.ok) {
        type GroupValue = string[] | { [key: string]: string }[];
        const groups: { [key: string]: GroupValue } = await getgroups.json();
        return { groups };
    }
    throw redirect(302, "/admin");
}) satisfies PageServerLoad;

export const actions = {
    addgroup: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get("name");
        await fetch(`${apiUrl}/user/admin/groups/add`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
    deletegroup: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get("name");
        await fetch(`${apiUrl}/user/admin/groups/remove`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                name,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
    changeperms: async ({ cookies, request }) => {
        const data = await request.formData();
        const name = data.get("name");
        const login = data.get("login");
        const admin = data.get("admin");
        const adminpage = data.get("adminpage");
        const user = data.get("user");
        const users = data.get("users");
        const groups = data.get("groups");
        const servers = data.get("servers");
        const workers = data.get("workers");
        const eggs = data.get("eggs");
        const sitesettings = data.get("sitesettings");
        const perms: string[] = [];
        if (login) perms.push("login");
        if (admin) perms.push("admin");
        if (adminpage) perms.push("adminpage");
        if (user) perms.push(user.toString());
        if (users) perms.push(users.toString());
        if (groups) perms.push(groups.toString());
        if (servers) perms.push(servers.toString());
        if (workers) perms.push(workers.toString());
        if (eggs) perms.push(eggs.toString());
        if (sitesettings) perms.push(sitesettings.toString());
        await fetch(`${apiUrl}/user/admin/groups/setperms`, {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${cookies.get("session")}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                group: name?.toString(),
                perms,
            }),
        });
        throw redirect(302, new URL(request.url).pathname);
    },
} satisfies Actions;
