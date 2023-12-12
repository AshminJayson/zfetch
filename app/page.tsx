"use client";
import Search from "./search";
import { appWindow } from "@tauri-apps/api/window";

appWindow.setFocus();

export default function Home() {
    return (
        <main className="flex flex-col min-h-screen p-5 gap-4">
            <p>&gt; Z Fetch</p>
            <Search />
        </main>
    );
}
