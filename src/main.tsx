import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import './index.css';
import {Toaster} from "@/components/ui/sonner.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <>
            <App/>
            <Toaster richColors={true} closeButton={true}/>
        </>
    </React.StrictMode>,
);
