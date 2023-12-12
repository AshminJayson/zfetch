"use client";
import { useEffect, useState } from "react";
import Search from "./search";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";

export default function Home() {
    const [appWindow, setAppWindow] = useState<WebviewWindow | null>(null);

    // Import appWindow and save it inside the state for later usage
    async function setupAppWindow() {
        const appWindow = (await import("@tauri-apps/api/window")).appWindow;
        setAppWindow(appWindow);
        appWindow.setFocus();
    }

    const closeApp = () => {
        if (appWindow) appWindow.close();
    };

    useEffect(() => {
        setupAppWindow();
    }, []);
    return (
        <main className="min-h-screen">
            <div
                data-tauri-drag-region="true"
                className="p-5 cursor-pointer text-sm font-extrabold flex w-full justify-between"
            >
                <p>
                    <span className="text-blue-500">_</span>Z Fetch
                </p>
                {/* <button onClick={closeApp}>Close</button> */}
            </div>
            <div className="px-5 flex flex-col">
                <Search />
            </div>
        </main>
    );
}
