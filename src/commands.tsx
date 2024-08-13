import { invoke } from '@tauri-apps/api/core'
import { notifications } from '@mantine/notifications'
import { IconCheck, IconX } from "@tabler/icons-react"
import { confirm } from '@tauri-apps/plugin-dialog'

export const set_directory = async (directory: string) => {
    try {
        let res = await invoke('set_directory', { directory: directory })
        console.log(res)
        notifications.show({
            id: "directory_set",
            message: "Directory set successfull",
            icon: <IconCheck size="1.1rem" />,
            autoClose: 1000,
            color: "teal",
        })
    } catch (err) {
        console.log(`error`, err);
        notifications.show({
            id: "directory_not_set",
            title: "Directory not found",
            message: "The specified directory was not found",
            icon: <IconX size="1.1rem" />,
            color: "red",
        })
        return false;
    }
    return true;
}

export const fetch_solved = async () => {
    try {
        await invoke('fetch_solved');
        notifications.show({
            id: "solved_fetch",
            message: "Solved problems fetched successfully",
            icon: <IconCheck size="1.1rem" />,
            autoClose: 1000,
            color: "teal"
        });
    } catch (err) {
        let create_confirm = await confirm('create solved.json?', { title: "File not found" });
        if (create_confirm) {
            return await create_solved();
        }
        return false;
    }
    return true;
}

export const create_solved = async () => {
    try {
        await invoke('create_solved');
        notifications.show({
            id: "solved_created",
            message: "created solved.json file",
        });
    } catch (err) {
        console.log(err)
        notifications.show({
            id: "solved_not_created",
            message: "Failed to create solved.json file",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });

        return false;
    }
    return true;
}

export const get_problemset = async () => {
    try {
        let res = await invoke('get_problemset');
        console.log(res);
        notifications.show({
            id: "problems_got",
            message: "Problems fetched successfully",
            icon: <IconCheck size="1.1rem" />,
            autoClose: 1000,
            color: "teal"
        });
    } catch (err) {
        console.log(`error`, err);
        notifications.show({
            id: "problems_not_got",
            message: "Failed to fetch problems please check your internet connection",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
        return false;
    }
    return true;
}

export const set_rating = async (rating: [number, number], tags: string[]) => {
    try {
        await invoke('set_rating', { min: rating[0], max: rating[1] })
        await invoke('set_tags', { tags: tags })
        notifications.show({
            id: "ratings_set",
            message: "Filters set successfull",
            icon: <IconCheck size="1.1rem" />,
            autoClose: 1000,
            color: "teal"
        });
    } catch (err) {
        console.log('error while setting filter');
        notifications.show({
            id: "rating_not_set",
            message: "Error encountered while setting filters",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
        return false;
    }
    return true;
}

export const get_problem = async () => {
    try {
        let res = await invoke('get_problem');
        return res;
    } catch (err) {
        notifications.show({
            id: "problem_not_got",
            message: "Error while getting problem",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const next_problem = async () => {
    try {
        await invoke('next_problem');
    } catch (err) {
        notifications.show({
            id: "no_next",
            message: "No next problem",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const prev_problem = async () => {
    try {
        await invoke('prev_problem');
    } catch (err) {
        notifications.show({
            id: "no_prev",
            message: "No prev problem",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const sort_problems = async (sorting: string) => {
    try {
        await invoke('sort_problems', { sortBy: sorting });
    } catch (err) {
        console.log(err)
        notifications.show({
            id: "no_sort",
            message: "Cannot sort problems",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
        return false;
    }
    return true;
}

export const get_testcases = async (contest_id: number, index: string) => {
    let res;
    try {
        res = await invoke('get_testcase', { contestId: contest_id, index: index })
    } catch (e) {
        console.log(`error ${e}`)
        try {
            res = await invoke('fetch_testcase', { contestId: contest_id, index: index })
        } catch (err) {
            console.log(`error ${err}`)
            notifications.show({
                id: "no_testcase",
                message: "Cannot get testcase",
                icon: <IconX size="1.1rem" />,
                color: "red"
            });
        }
    }
    return res;
}

export const problem_solved = async () => {
    try {
        await invoke('problem_solved');
    } catch {
        notifications.show({
            id: "not_solved",
            message: "Error ocurred while saving solved status",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const set_hide_solved = async (value: boolean) => {
    try {
        await invoke('set_hide_solved', { value: value })
    } catch (err) {
        console.log(err);
        notifications.show({
            id: "hide_not_set",
            message: "Error ocurred while setting hide solved",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const create_file = async () => {
    try {
        await invoke('create_file');
    } catch (err) {
        console.log(err);
        notifications.show({
            id: "file_not_create",
            message: "Error while creating solution file",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const open_link = async () => {
    try {
        await invoke('open_link');
    } catch (err) {
        console.log(err);
        notifications.show({
            id: "link_not_open",
            message: "Error while opening link",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
}

export const judge = async () => {
    let res;
    try {
        res = await invoke('judge');
    } catch (err) {
        console.log(err);
        notifications.show({
            id: "file_not_judged",
            message: "Error while testing solution",
            icon: <IconX size="1.1rem" />,
            color: "red"
        });
    }
    return res;
}