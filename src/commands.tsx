import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";
import { IconCheck, IconX } from "@tabler/icons-react";
import { Language, Problem, Verdict } from "./Languages.ts";

export const set_directory = async (directory: string) => {
    try {
        await invoke("set_directory", { directory: directory });
        return true;
    } catch (e) {
        console.log(e);
        notifications.show({
            id: "directory_not_set",
            title: "Directory not found",
            message: "The specified directory was not found",
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_directory = async () => {
    try {
        return await invoke("get_directory") as string;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_directory",
            message: "Cannot get the directory",
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return "";
    }
};

export const set_language = async (language_id: number) => {
    try {
        await invoke("set_language", { languageId: language_id });
        notifications.show({
            id: "language_set",
            message: "language set successfully",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_language",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_language_dir = async (language_id: number) => {
    try {
        return await invoke("get_language_dir", { languageId: language_id }) as string;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_language_dir",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return "";
    }
};

export const set_language_dir = async (language_id: number, dir: string) => {
    try {
        await invoke("set_language_dir", { languageId: language_id, dir: dir });
        notifications.show({
            id: "language_set",
            message: "language dir set successfully",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
        return true
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_language_dir",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_language = async () => {
    try {
        return (await invoke("get_language")) as number;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_language",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return 0;
    }
};

export const get_languages = async () => {
    try {
        return (await invoke("get_languages")) as Language[];
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_languages",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return [];
    }
}

export const set_problem = async (problem: Problem) => {
    try {
        await invoke("set_problem", { problem: problem });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_problem",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_problem = async () => {
    try {
        return (await invoke("get_problem")) as Problem;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_problem",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return null;
    }
};

export const set_verdicts = async (verdicts: Verdict[]) => {
    try {
        await invoke("set_verdicts", { verdicts: verdicts });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_verdicts",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_base_url = async () => {
    try {
        return (await invoke("get_base_url")) as string;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_problem",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return "";
    }
};

export const set_base_url = async (url: string) => {
    try {
        await invoke("set_base_url", { url: url });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_set_base_url",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const get_verdicts = async () => {
    try {
        return (await invoke("get_verdicts")) as Verdict[];
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_get_verdicts",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return null;
    }
};

export const run = async () => {
    try {
        return (await invoke("test")) as null;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_run",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return null;
    }
};

export const submit = async () => {
    try {
        await invoke("submit_solution");
        notifications.show({
            id: "language_set",
            message: "submitting on codeforces...",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_submit",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
    }
    return null;
};

export const create_file = async () => {
    try {
        await invoke("create_file");
        notifications.show({
            id: "file_created",
            message: "file created",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_create_file",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const open_file = async () => {
    try {
        await invoke("open_file");
        notifications.show({
            id: "file_opened",
            message: "file opened in editor",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_open_file",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};

export const save_state = async () => {
    try {
        await invoke("save_state");
        return true;
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_save_state",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return false;
    }
};
