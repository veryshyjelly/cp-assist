import {Center, Flex, Image, Select, Text} from "@mantine/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {TauriEvent} from "@tauri-apps/api/event";
import {Languages} from "./Languages.ts";

const appWindow = getCurrentWindow()

const TitleBar = () => {
    const [isFocused, setIsFocused] = useState(true);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        getCurrentWindow().listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
            invoke("save_state").then(() => {
            }).catch(e => console.error(e));
        });
        window.addEventListener("focus", () => setIsFocused(true));
        window.addEventListener("blur", () => setIsFocused(false));
    }, [])


    return (
        <Flex h={40} data-tauri-drag-region
              style={{backgroundColor: isFocused ? "#2b2d30" : "#3c3f41"}}>

            <Center h={33} w={33} mx={3} my={"auto"} className="rounded-full hover:bg-[#484b4d]">
                <Image src="back.svg" h={25}/>
            </Center>
            <Center h={33} w={33} mx={3} my={"auto"} className="rounded-full hover:bg-[#484b4d]">
                <Image src="refresh.svg" h={34}/>
            </Center>

            <Flex ml={"29%"} mr={"auto"}>
                {loading &&
                    <Center h={35} w={180} my={"auto"} mx={5}
                            className="rounded-md bg-[#484b4d] cursor-pointer" onClick={() => {
                        setLoading(false)
                    }}>
                        <Image src="pending.gif" h={60}/>
                        <Text mx={10} c={"white"}>Running</Text>
                    </Center>
                }
                {!loading &&
                    <>
                        <Center h={35} w={80} my={"auto"} mx={5}
                                className="bg-black/15 rounded-md hover:bg-[#484b4d] cursor-pointer" onClick={() => {
                            setLoading(true)
                        }}>
                            <Image src="play.svg" h={25} ml={10}/>
                            <Text mx={10} c={"white"}>Run</Text>
                        </Center>
                        <Center h={35} my={"auto"} w={115}
                                className="bg-black/15 rounded-md hover:bg-[#484b4d] cursor-pointer" onClick={() => {
                        }}>
                            <Image src="submit.svg" h={25} ml={10}/>
                            <Text mx={10} c={"#28c244"}>Submit</Text>
                        </Center>
                    </>}
            </Flex>

            <Center h={34} w={34} my={"auto"} mx={2} title={"Create File"}
                    className="rounded-md hover:bg-[#484b4d] cursor-pointer" onClick={() => {
            }}>
                <Image src="create_file.svg" h={22}/>
            </Center>
            <Select
                h={35}
                w={130}
                my={"auto"}
                variant="light"
                c={"white"}
                defaultValue={"python"}
                className={"bg-black/15 rounded-md"}
                data={Languages}
            />

            <Center h={40} w={36} className="rounded-md hover:bg-[#484b4d]" ml={20}>
                <Image src="settings.svg" h={28}/>
            </Center>

            <Flex ml={10}>
                <Center h={40} w={36} className="hover:bg-[#484b4d]" onClick={() => appWindow.minimize()}>
                    <Image src="minimize.svg"/>
                </Center>
                <Center h={40} w={36}><Image src="maximize.svg"/></Center>
                <Center h={40} w={36} className="hover:bg-[#e81123]" onClick={() => appWindow.close()}>
                    <Image src="close.svg"/>
                </Center>
            </Flex>
        </Flex>
    );
}

export default TitleBar;