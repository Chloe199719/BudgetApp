import Carrousel from "@/components/Atoms/Carrousel";

import { redirect } from "next/navigation";

import { cookies } from "next/headers";
import { getUserData } from "@/app/layout";
import ResetPasswordForm from "@/components/auth/password/ResetPasswordForm";

type Props = {};
async function Page({}: Props) {
    const cookieStore = cookies();
    const sessionId = cookieStore.get("sessionid");
    const user = await getUserData(sessionId?.value);
    if (user) {
        redirect("/");
    }
    return (
        <main className="flex flex-1">
            <section className="w-full grid grid-cols-1  md:grid-cols-6  justify-items-center p-10">
                <ResetPasswordForm />
                <Carrousel />
            </section>
        </main>
    );
}
export default Page;
