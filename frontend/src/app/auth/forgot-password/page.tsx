import { cookies } from "next/headers";
import { getUserData } from "../../layout";
import { redirect } from "next/navigation";
import Carrousel from "@/components/Atoms/Carrousel";
import ForgetPasswordForm from "@/components/auth/forgotPassword/forgotPassword";

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
                <ForgetPasswordForm />
                <Carrousel />
            </section>
        </main>
    );
}
export default Page;
