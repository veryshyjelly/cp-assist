import {Box, Flex, Group, Image, Select, Stack, Text, Textarea} from "@mantine/core";
import {useEffect, useState} from "react";
import {LogicalSize} from "@tauri-apps/api/window";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";

const appWindow = getCurrentWebviewWindow()

const MyTextarea = ({title, value, mx}: { title: string, value: string, mx: number | string | undefined }) => {
    return <Box
        className={`h-[25rem] w-[31%] font-[500] bg-[#282828]
                        text-3xl border border-gray-600 relative text-center select-none font-mono
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

const Home = () => {
    let [_problem, _setProblem] = useState<{
        contest_id: number;
        memory_limit: number;
        problem_id: string;
        time_limit: number;
        title: string;
    } | null>(null);
    let [caseIndex, setCaseIndex] = useState(0);
    let [finalVerdict, _setFinalVerdict] = useState("Accepted" as string);
    let [verdicts, _setVerdicts] = useState<
        {
            input: string;
            output: string;
            answer: string;
            status: string;
            time: number;
            memory: number;
        }[]
    >([{input: "4\n0 3\n2 5\n4 2\n4 0\n", output: "6", answer: "6", memory: 500, status: "WA", time: 1}]);

    useEffect(() => {
        appWindow.setSize(new LogicalSize(1000, 650)).then(null);
    }, [])

    return (<Stack>
        <Flex className={"mt-5 justify-center content-center"}>
            <Box
                my={"auto"}
                onClick={() => {
                }}
                className="p-3 mt-auto rounded-xl hover:bg-[#484b4d] active:drop-shadow-2xl border border-gray-500 cursor-pointer"
            >
                <Image src={"/prev.svg"} w={15} h={15}/>
            </Box>

            <Text w={500} fz={32} fw={600} c={"white"}
                  className={"select-none tracking-widest cursor-pointer text-center"}>
                Valid Parenthesis
            </Text>
            <Box
                my={"auto"}
                onClick={() => {
                }}
                className="p-3 mt-auto rounded-xl hover:bg-[#484b4d] active:drop-shadow-2xl border border-gray-500 cursor-pointer"
            >
                <Image src={"/next.svg"} w={15} h={15}/>
            </Box>

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
                            (verdicts[x]?.status === "AC" ? "✔️" : "❌"),
                        value: `${x + 1}`,
                    }))}
                    w={150} pl={10} py={4} bg={"#2b2d30"}
                    className={"rounded-md"}
                    checkIconPosition={"right"}
                    defaultValue={"1"}
                    allowDeselect={false}
                />
                <Text fz={26} fw={600} ml={"md"} className={"my-auto tracking-wider font-mono"}
                      style={{
                          color: finalVerdict === "Accepted" ? "#2cad40" : finalVerdict === "Wrong Answer" ? "red" : "gray",
                      }}
                >
                    {finalVerdict}
                </Text>

            </Group>

            <Group className="justify-between w-[100%]">
                <MyTextarea title={"Input"} value={verdicts[caseIndex]?.input ?? ""} mx={0}/>
                <MyTextarea title={"Answer"} value={verdicts[caseIndex]?.answer ?? ""} mx={"auto"}/>
                <MyTextarea title={"Output"} value={verdicts[caseIndex]?.output ?? ""} mx={0}/>
            </Group>
        </Stack>
    </Stack>)
}


export default Home;