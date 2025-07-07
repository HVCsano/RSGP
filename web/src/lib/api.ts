export const apiUrl = process.env.NODE_ENV === "development"
    ? "http://localhost:3000"
    : process.env.API_URL as string;

export type PermTypes = "Read" | "Write";
export type Permissions = string | { [key: string]: PermTypes };

export function checkAdvancedPerms(
    perms: Permissions[],
    perm: string,
    type: PermTypes[],
) {
    if (perms.includes("Admin")) return true;
    for (const t of type) {
        const permsome = perms.some((p) =>
            typeof p === "object" && p[perm] === t
        );
        if (permsome) {
            return true;
        }
    }
    return false;
}
