import { getUserData } from "@/app/layout";
import ProfilePage from "@/components/ProfilePage/PorfilePage";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

type Props = {};
async function page({}: Props) {
    const cookieStore = cookies();
    const sessionId = cookieStore.get("sessionid");
    const user = await getUserData(sessionId?.value);
    if (!user) {
        redirect("/");
    }
    return (
        <div className="p-4 bg-gray-300 dark:bg-gray-950">
            <ProfilePage />
        </div>
    );
}
export default page;
