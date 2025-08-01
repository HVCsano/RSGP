export interface Egg {
    install: { dependency_installs: string[][]; egg_installs: string[][] };
    running: {
        start_command: string;
        running_text: string;
    };
    version: string;
    upstream?: string;
}

export type ServerStates =
    | "Created"
    | "Installing"
    | "Stopped"
    | "Running";
