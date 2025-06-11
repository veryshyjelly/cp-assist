import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import '@mantine/core/styles.css'
import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <MantineProvider defaultColorScheme={"dark"}>
            <Notifications limit={5} pos={"absolute"} className={"bottom-5 left-3"} w={"auto"} />
            <App />
        </MantineProvider>
    </React.StrictMode>,
);
