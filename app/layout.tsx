import type { Metadata } from "next";
import { IBM_Plex_Mono } from "next/font/google";
import "./globals.css";

const ibmPlexMono = IBM_Plex_Mono({
    weight: ["100", "200", "300", "400", "500", "600", "700"],
    style: ["normal", "italic"],
    subsets: ["latin"],
});

export const metadata: Metadata = {
    title: "Z Fetch",
    description: "Persistent clipboard with fuzzy search",
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body className={`${ibmPlexMono.className}`}>{children}</body>
        </html>
    );
}
