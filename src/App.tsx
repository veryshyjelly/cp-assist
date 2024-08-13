import "./App.css";
import {Box} from "@mantine/core";
import TitleBar from "./Titlebar.tsx";
import LandingPage from "./LandingPage.tsx";
import {invoke} from "@tauri-apps/api/core";
import {useEffect, useState} from "react";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {LogicalSize} from "@tauri-apps/api/window";
import Home from "./Home.tsx";
const appWindow = getCurrentWebviewWindow()

function App() {
    const [directory, setDirectory] = useState("xyz");

    useEffect(() => {
        invoke("get_directory")
            .then((dir) => {
                if (dir !== "")
                    appWindow.setSize(new LogicalSize(1600, 900)).then(null);
                setDirectory(dir as string);
            })
            .catch((e) => console.error(e));
    });

  return (
      <Box
          className="bg-[#1e1f22] border border-[#3c3f41]"
          style={{ height: "100%", width: "100%", position: "fixed" }}
      >
        <TitleBar/>
          {directory === "" && <LandingPage setDirectory={setDirectory}/>}
          {directory !== "" && <Home/>}
    </Box>
  );
}

export default App;
