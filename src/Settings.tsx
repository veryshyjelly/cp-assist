import {Button, Group, Select, Text, TextInput} from "@mantine/core";
import {IconCheck} from "@tabler/icons-react";
import {useEffect, useState} from "react";
import {get_base_url, get_language_dir, get_languages, set_base_url, set_language_dir} from "./commands.tsx";

const Settings = ({close} : {close: () => void}) => {
    const [language, setLanguage] = useState("0");
    const [languages, setLanguages] = useState<{ value: string, label: string }[]>([]);
    const [defaultDir, setDefaultDir] = useState("");
    const [baseUrl, setBaseUrl] = useState("");

    const onChangeLanguage = (v: string | null) => {
        if (v === null) return;
        setLanguage(v);
        get_language_dir(parseInt(v)).then(x => setDefaultDir(x));
    }

    const onSaveDir = () => {
        set_language_dir(parseInt(language), defaultDir).then(() => close())
    }

    const onSaveUrl = () => {
        set_base_url(baseUrl).then(() => close());
    }

    useEffect(() => {
        get_base_url().then(v => setBaseUrl(v));
        get_languages().then(v => setLanguages(v.map(x => {
            return {value: x.id.toString(), label: x.name}
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
                    comboboxProps={{width: 250}}
                    renderOption={({option, checked}) => <Group>
                        <Text fz={18} fw={"500"} className={"tracking-wider"}>{option.label}</Text> {checked &&
                        <IconCheck style={{marginInlineStart: 'auto'}}/>} </Group>}
            />
            <TextInput value={defaultDir} onChange={(event) => setDefaultDir(event.currentTarget.value)}/>
            <Button onClick={onSaveDir}>
                Save
            </Button>
        </Group>
        <Text mt={10} c={"#acacac"}>Set url for judge0</Text>
        <Group>
            <TextInput value={baseUrl} w={"80%"} onChange={(event) => setBaseUrl(event.currentTarget.value)}/>
            <Button onClick={onSaveUrl} ml={"auto"}>
                Save
            </Button>
        </Group>
    </>
}

export default Settings;