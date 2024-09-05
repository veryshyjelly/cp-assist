import { Button, Group, Select, Text, TextInput } from "@mantine/core";
import { IconCheck } from "@tabler/icons-react";
import { useEffect, useState } from "react";
import { get_language, get_language_dir, get_languages, get_open_with, set_language_dir, set_open_with } from "./commands.tsx";

const Settings = ({ close }: { close: () => void }) => {
    const [language, setLanguage] = useState("0");
    const [languages, setLanguages] = useState<{ value: string, label: string }[]>([]);
    const [defaultDir, setDefaultDir] = useState("");
    const [openApplication, setOpenApplication] = useState("");

    const onChangeLanguage = (v: string | null) => {
        if (v === null) return;
        setLanguage(v);
        get_language_dir(parseInt(v)).then(x => setDefaultDir(x));
    }

    const onSaveDir = () => {
        set_language_dir(parseInt(language), defaultDir).then(() => close())
    }

    const onSaveOpenApp = () => {
        set_open_with(openApplication).then(() => close());
    }


    useEffect(() => {
        get_open_with().then(v => setOpenApplication(v));
        get_language().then(v => onChangeLanguage(v.toString()));
        get_languages().then(v => setLanguages(v.map(x => {
            return { value: x.id.toString(), label: x.name }
        })))
    }, [])

    return <>
        <Text my={2} c={"#acacac"}>Set directory for file creation</Text>
        <Group>
            <Select h={35} my={"auto"} w={250}
                variant="light" c={"white"}
                defaultValue={"0"} data={languages} value={language}
                className={"bg-black/15 rounded-md z-10"}
                onChange={onChangeLanguage} allowDeselect={false}
                comboboxProps={{ width: 250 }}
                renderOption={({ option, checked }) => <Group>
                    <Text fz={18} fw={"500"} className={"tracking-wider"}>{option.label}</Text> {checked &&
                        <IconCheck style={{ marginInlineStart: 'auto' }} />} </Group>}
            />
            <TextInput value={defaultDir} onChange={(event) => setDefaultDir(event.currentTarget.value)} />
            <Button onClick={onSaveDir}>
                Save
            </Button>
        </Group>
        <Text mt={20} mb={2} c={"#acacac"}>Set application to open the file</Text>
        <Group>
            <TextInput value={openApplication} w={"80%"} onChange={(event) => setOpenApplication(event.currentTarget.value)} />
            <Button onClick={onSaveOpenApp} ml={"auto"}>
                Save
            </Button>
        </Group>
    </>
}

export default Settings;