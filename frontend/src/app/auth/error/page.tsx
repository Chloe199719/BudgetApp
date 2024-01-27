import ErrorPage from "@/components/auth/error/ErrorPage";
import { Button } from "@/components/ui/button";
import Link from "next/link";

type Props = {};
function page({}: Props) {
    return (
        <div className="flex flex-col gap-20 items-center justify-center flex-1 bg-gradient-to-tr from-lime-200 to-blue-400">
            <div className="flex flex-col gap-20 border-black border-2 p-20 rounded-3xl items-center">
                <h2 className="text-2xl md:text-7xl text-pink-500">
                    There was a problem with Auth
                </h2>
                <ErrorPage />

                <Link
                    className="bg-blue-700 text-gray-100 hover:bg-cyan-800 p-4 rounded-xl active:-translate-u-2 w-full"
                    href="/"
                >
                    Back to Home
                </Link>
            </div>
        </div>
    );
}
export default page;
