import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";
import { IconCheck, IconX } from "@tabler/icons-react";
import {Language, Problem, Verdict} from "./Languages.ts";

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
        await invoke("set_language", { language_id: language_id });
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
        notifications.show({
            id: "problem_set",
            message: "problem set",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
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
        notifications.show({
            id: "verdicts_set",
            message: "verdicts set",
            icon: <IconCheck size="1.1rem" />,
            color: "teal",
        });
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
        return (await invoke("run")) as Verdict[];
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
        return (await invoke("submit")) as Verdict[];
    } catch (e) {
        console.error(e);
        notifications.show({
            id: "cannot_submit",
            message: e as string,
            icon: <IconX size="1.1rem" />,
            color: "red",
        });
        return null;
    }
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
