import type { Metadata } from "next";
import { Inter, Roboto_Mono } from "next/font/google";
import "./globals.css";
import { Providers } from "@/lib/redux/providers";
import { cookies } from "next/headers";
import axios from "axios";
import Navbar from "@/components/navbar/Navbar";
import { Toaster } from "@/components/ui/toaster";
import { ThemeProvider } from "@/components/Theme-Provider";
import axiosInstance from "@/lib/api/axios";
import MainBar from "@/components/navbar/MainBar";
import Footer from "@/components/footer/Footer";
const inter = Inter({ subsets: ["latin"] });
const roboto_mono = Roboto_Mono({
    subsets: ["latin"],
    display: "swap",
    variable: "--font-roboto-mono",
});
export const metadata: Metadata = {
    title: "Budget App",
    description: "Ai powered budgeting app",
};

export default async function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const cookieStore = cookies();
    const sessionId = cookieStore.get("sessionid");
    const userData = await getUserData(sessionId?.value);

    return (
        <Providers userData={userData}>
            <html lang="en" suppressHydrationWarning={true}>
                <body
                    className={`${inter.className} min-h-screen flex flex-col`}
                >
                    <ThemeProvider
                        attribute="class"
                        defaultTheme="system"
                        enableSystem
                        disableTransitionOnChange
                    >
                        <MainBar />
                        {children}
                        <Footer />
                        <Toaster />
                    </ThemeProvider>
                </body>
            </html>
        </Providers>
    );
}
export async function getUserData(sessionId?: string) {
    try {
        console.log("sessionId", sessionId);
        if (!sessionId) return;
        const res = await axios.get(
            `${process.env.NEXT_PUBLIC_BACKEND_URL}/users/current-user`,
            {
                withCredentials: true,
                headers: { Cookie: `sessionid=${sessionId}` },
            },
        );
        console.log("res", res);
        return res.data as CurrentUserData;
    } catch (error) {
        return;
    }
}

export type CurrentUserData = {
    id: string;
    email: string;
    display_name: string;
    unique_name: string;
    is_active: boolean;
    is_staff: boolean;
    is_superuser: boolean;
    thumbnail?: string | null;
    data_joined: string;
    profile: {
        id: string;
        phone_number?: string | null;
        about_me?: string | null;
        pronouns?: string | null;
        avatar_link?: string | null;
        birth_date?: string | null;
        github_link?: string | null;
    };
};
