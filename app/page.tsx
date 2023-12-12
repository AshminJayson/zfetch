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

    useEffect(() => {
        setupAppWindow();
    }, []);
    return (
        <main className="flex flex-col min-h-screen p-5 gap-4">
            <button
                onClick={() => {
                    if (appWindow == null) {
                        console.log("no window");
                        return;
                    }
                    console.log("dragging");
                    appWindow.startDragging();
                }}
            >
                Some button
            </button>
            <p>&gt; Z Fetch</p>
            <Search />
        </main>
    );
}
