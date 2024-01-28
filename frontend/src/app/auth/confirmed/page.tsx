import { Link } from "lucide-react";

type Props = {};
function page({}: Props) {
    return (
        <section className="flex-1 flex items-center justify-center flex-col gap-10">
            <h2 className="text-4xl text-green-600">
                Thank you for Confirming Your Email
            </h2>
            <h3 className="text-2xl text-green-400">You now Login</h3>
            <Link className="bg-blue-500 p-4 w-full" href="/auth/login">
                Login
            </Link>
        </section>
    );
}
export default page;
