import "./App.css";
import {Box,  Modal } from "@mantine/core";
import TitleBar from "./Titlebar.tsx";
import LandingPage from "./LandingPage.tsx";
import {useEffect, useState} from "react";
import Home from "./Home.tsx";
import {Problem, Verdict} from "./Languages.ts";
import {listen} from "@tauri-apps/api/event";
import {get_directory, get_problem, get_verdicts, set_problem, set_verdicts} from "./commands.tsx";
import {useDisclosure} from "@mantine/hooks";
import Settings from "./Settings.tsx";

function App() {
    const [directory, setDirectory] = useState("");
    const [problem, setProblem] = useState<Problem | null>(null);
    const [verdicts, setVerdicts] = useState<Verdict[]>([]);
    const [opened, { open, close }] = useDisclosure(false);

    useEffect(() => {
        get_directory().then(dir => setDirectory(dir));
        get_problem().then(pro => setProblem(pro));
        get_verdicts().then(ver => setVerdicts(ver || []))
        listen<Problem>('set-problem', event =>
            set_problem(event.payload).then(() => setProblem(event.payload)));
        listen<Verdict[]>('set-verdicts', event =>
            set_verdicts(event.payload).then(() => setVerdicts(event.payload)));
    }, []);

    return (
        <Box
            className="bg-[#1e1f22] border border-[#3c3f41]"
            style={{height: "100%", width: "100%", position: "fixed"}}
        >
            <Modal opened={opened} onClose={close} title="Settings" centered size={"auto"}>
                <Settings close={close}/>
            </Modal>
            <TitleBar setDirectory={setDirectory} directory={directory} open={open}/>
            {directory === "" && <LandingPage setDirectory={setDirectory}/>}
            {directory !== "" && <Home problem={problem} verdicts={verdicts}/>}
        </Box>
    );
}

export default App;
