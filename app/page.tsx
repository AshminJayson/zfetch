"use client";
import { useEffect, useState } from "react";
import Search from "./search";
import { LogicalPosition, WebviewWindow } from "@tauri-apps/api/window";

export default function Home() {
    const [appWindow, setAppWindow] = useState<WebviewWindow | null>(null);

    // Import appWindow and save it inside the state for later usage
    async function setupAppWindow() {
        const _appwindow = (await import("@tauri-apps/api/window")).appWindow;
        setAppWindow(_appwindow);

        // retrievedAppWindow.setPosition(new LogicalPosition(100, 0));
        _appwindow.setFocus();
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
                className="px-5 py-4 cursor-pointer text-sm font-extrabold flex w-full justify-between"
            >
                <p data-tauri-drag-region="true">
                    <span className="text-blue-500">_</span>Z Fetch
                </p>
                <div className="flex gap-4">
                    <button onClick={closeApp} className="text-red-500">
                        X
                    </button>
                </div>
            </div>
            <div className="px-5 flex flex-col">
                <Search />
            </div>
        </main>
    );
}
