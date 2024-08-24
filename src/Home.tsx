import {Box, Center, Flex, Group, Select, Stack, Text, Textarea} from "@mantine/core";
import {useEffect, useState} from "react";
import {LogicalSize} from "@tauri-apps/api/window";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {Problem, Verdict} from "./Languages.ts";

const appWindow = getCurrentWebviewWindow()

const MyTextarea = ({title, value, mx}: { title: string, value: string, mx: number | string | undefined }) => {
    return <Box h={"90%"} w={"31%"} fw={500} bg={"#282828"}
                className={`text-3xl border border-gray-600 relative text-center select-none font-mono
                            rounded-md tracking-widest pt-1`}
                mx={mx}
    >
        {title}
        <Textarea
            className={`h-[91%] w-full px-2 top-10 bg-[#3e3e3e]/50 rounded-md absolute`}
            variant="unstyled"
            maxRows={15}
            autosize
            value={value}/>
    </Box>
}

const Home = ({problem, verdicts}: {problem: Problem|null, verdicts: Verdict[]}) => {
    let [caseIndex, setCaseIndex] = useState(0);

    useEffect(() => {
        appWindow.setSize(new LogicalSize(1000, 650)).then(null);
    }, [])

    return (problem?.title ? <Stack h={"95%"}>
        <Flex className={"mt-5 justify-center content-center"}>
            <Text w={500} fz={32} fw={600} c={"white"}
                  className={"select-none tracking-widest text-center"}>
                {problem?.title}
            </Text>
        </Flex>

        <Stack c={"white"} w={"95%"} mt={"10"} mx={"auto"} px={40} className={"text-2xl h-full rounded-md"}>
            {/*Heading Area*/}
            <Group my={5}>
                <Select
                    variant="unstyled"
                    onChange={(v) => setCaseIndex(parseInt(v ?? "1") - 1)}
                    data={Array.from(Array(verdicts.length).keys()).map((x) => ({
                        label:
                            `Case ${x + 1} ` +
                            (verdicts[x]?.status_id === 3 ? "✔️" : verdicts[x]?.status_id < 3 ? "😶" : "❌"),
                        value: `${x + 1}`,
                    }))}
                    w={150} pl={10} py={4} bg={"#2b2d30"}
                    className={"rounded-md"}
                    checkIconPosition={"right"}
                    defaultValue={"1"}
                    allowDeselect={false}
                />
                <Text fz={26} fw={600} ml={"md"} my={"auto"} ff={"monospace"} className={"tracking-wider"}
                      style={{
                          color: verdicts[caseIndex]?.status_id === 3 ? "#2cad40" : verdicts[caseIndex]?.status_id < 3 ? "gray" : "red",
                      }}
                >
                    {verdicts[caseIndex]?.status}
                </Text>
                {verdicts[caseIndex]?.time && <>
                    <Text ff={"monospace"} c={"#fcfcfc"} fz={20}>{verdicts[caseIndex]?.time*1000 + " ms"}</Text>
                    <Text ff={"monospace"} c={"#fcfcfc"} fz={20}>{Math.floor(verdicts[caseIndex]?.memory/1024) + " Mb"}</Text>
                </>}
            </Group>

            <Group h={"90%"} w={"100%"}>
                <MyTextarea title={"Input"} value={verdicts[caseIndex]?.input ?? ""} mx={0}/>
                <MyTextarea title={"Answer"} value={verdicts[caseIndex]?.answer ?? ""} mx={"auto"}/>
                <MyTextarea title={"Output"} value={verdicts[caseIndex]?.output ?? ""} mx={0}/>
            </Group>
        </Stack>

    </Stack> : <Center h={"90%"}>
            <Text c={"#acacac"} fz={32} className={"tracking-wider"}>Select problem from competitive companion</Text>
    </Center>)
}


export default Home;