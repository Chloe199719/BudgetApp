import Link from "next/link";
import { SVGProps } from "react";

type Props = {};
function LandingPage({}: Props) {
    return (
        <>
            <section className="w-full py-12 md:py-24 lg:py-32">
                <div className="container px-4 md:px-6">
                    <div className="flex flex-col items-center justify-center space-y-4 text-center">
                        <div className="space-y-2">
                            <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                                Manage your finances with ease.
                            </h1>
                            <p className="max-w-[900px] text-gray-500 md:text-xl/relaxed lg:text-base/relaxed xl:text-xl/relaxed dark:text-gray-400">
                                Our budgeting tool helps you understand your
                                spending and balance your life. Get started
                                today.
                            </p>
                        </div>
                        <div className="flex flex-col gap-2 min-[400px]:flex-row justify-center">
                            <Link
                                className="inline-flex h-10 items-center justify-center rounded-md bg-gray-900 px-8 text-sm font-medium text-gray-50 shadow transition-colors hover:bg-gray-900/90 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                                href="#"
                            >
                                Sign Up
                            </Link>
                            <Link
                                className="inline-flex h-10 items-center justify-center rounded-md border  border-gray-200 bg-white px-8 text-sm font-medium shadow-sm transition-colors hover:bg-gray-100 hover:text-gray-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50  dark:border-gray-800 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus-visible:ring-gray-300"
                                href="#"
                            >
                                Learn More
                            </Link>
                        </div>
                    </div>
                </div>
            </section>
            <section className="w-full py-12 md:py-24 lg:py-32 bg-gray-100 dark:bg-gray-800">
                <div className="container grid items-center justify-center gap-4 px-4 text-center md:px-6">
                    <div className="space-y-3">
                        <h2 className="text-3xl font-bold tracking-tighter md:text-4xl/tight">
                            Features
                        </h2>
                        <p className="max-w-[600px] text-gray-500 md:text-xl/relaxed lg:text-base/relaxed xl:text-xl/relaxed dark:text-gray-400">
                            Explore the amazing features that make our app stand
                            out.
                        </p>
                    </div>
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div className="flex flex-col items-center space-y-4">
                            <GoalIcon className="h-12 w-12" />
                            <h3 className="text-xl font-bold">
                                Budget Planning
                            </h3>
                            <p className="text-gray-500 dark:text-gray-400">
                                Plan your budget and track your expenses easily.
                            </p>
                        </div>
                        <div className="flex flex-col items-center space-y-4">
                            <ViewIcon className="h-12 w-12" />
                            <h3 className="text-xl font-bold">
                                Detailed Reports
                            </h3>
                            <p className="text-gray-500 dark:text-gray-400">
                                Get detailed reports on your spending habits.
                            </p>
                        </div>
                        <div className="flex flex-col items-center space-y-4">
                            <LockIcon className="h-12 w-12" />
                            <h3 className="text-xl font-bold">
                                Secure Transactions
                            </h3>
                            <p className="text-gray-500 dark:text-gray-400">
                                Your data is safe with us. We use the latest
                                encryption technologies.
                            </p>
                        </div>
                    </div>
                </div>
            </section>
            <section className="w-full py-12 md:py-24 lg:py-32">
                <div className="container grid items-center justify-center gap-4 px-4 text-center md:px-6 lg:gap-10">
                    <div className="space-y-3">
                        <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                            Trusted by thousands of users
                        </h2>
                        <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl/relaxed lg:text-base/relaxed xl:text-xl/relaxed dark:text-gray-400">
                            Join our community and start managing your finances
                            today.
                        </p>
                    </div>
                    <div className="divide-y rounded-lg border">
                        <div className="grid w-full grid-cols-3 items-stretch justify-center divide-x md:grid-cols-3">
                            <div className="mx-auto flex w-full items-center justify-center p-4 sm:p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                            <div className="mx-auto flex w-full items-center justify-center p-4 sm:p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                            <div className="mx-auto flex w-full items-center justify-center p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                        </div>
                        <div className="grid w-full grid-cols-3 items-stretch justify-center divide-x md:grid-cols-3">
                            <div className="mx-auto flex w-full items-center justify-center p-4 sm:p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                            <div className="mx-auto flex w-full items-center justify-center p-4 sm:p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                            <div className="mx-auto flex w-full items-center justify-center p-4 sm:p-8">
                                <img
                                    alt="User"
                                    className="aspect-[1/1] overflow-hidden rounded-full object-contain object-center"
                                    height="140"
                                    src="/placeholder.svg"
                                    width="140"
                                />
                            </div>
                        </div>
                    </div>
                    <div className="flex justify-center space-x-4">
                        <Link
                            className="inline-flex h-10 items-center justify-center rounded-md bg-gray-900 px-8 text-sm font-medium text-gray-50 shadow transition-colors hover:bg-gray-900/90 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                            href="#"
                        >
                            Sign Up
                        </Link>
                        <Link
                            className="inline-flex h-10 items-center justify-center rounded-md border  border-gray-200 bg-white px-8 text-sm font-medium shadow-sm transition-colors hover:bg-gray-100 hover:text-gray-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50  dark:border-gray-800 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus-visible:ring-gray-300"
                            href="#"
                        >
                            Learn more
                        </Link>
                    </div>
                </div>
            </section>
            <section className="w-full py-12 md:py-24 lg:py-32 border-t">
                <div className="container px-4 md:px-6">
                    <div className="grid gap-10 px-10 md:gap-16 lg:grid-cols-2">
                        <div className="space-y-4">
                            <div className="inline-block rounded-lg bg-gray-100 px-3 py-1 text-sm dark:bg-gray-800">
                                Budgeting
                            </div>
                            <h2 className="lg:leading-tighter text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl xl:text-[3.4rem] 2xl:text-[3.75rem]">
                                Take control of your finances.
                            </h2>
                            <Link
                                className="inline-flex h-9 items-center justify-center rounded-md bg-gray-900 px-4 py-2 text-sm font-medium text-gray-50 shadow transition-colors hover:bg-gray-900/90 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                                href="#"
                            >
                                Get Started
                            </Link>
                        </div>
                        <div className="flex flex-col items-start space-y-4">
                            <div className="inline-block rounded-lg bg-gray-100 px-3 py-1 text-sm dark:bg-gray-800">
                                Security
                            </div>
                            <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl/relaxed dark:text-gray-400">
                                Your data is safe with us. We use the latest
                                encryption technologies to ensure your
                                information is secure.
                            </p>
                            <Link
                                className="inline-flex h-9 items-center justify-center rounded-md border  border-gray-200 bg-white px-4 py-2 text-sm font-medium shadow-sm transition-colors hover:bg-gray-100 hover:text-gray-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950 disabled:pointer-events-none disabled:opacity-50  dark:border-gray-800 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus-visible:ring-gray-300"
                                href="#"
                            >
                                Learn More
                            </Link>
                        </div>
                    </div>
                </div>
            </section>
        </>
    );
}
export default LandingPage;
function LockIcon(props: SVGProps<SVGSVGElement>) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
            <path d="M7 11V7a5 5 0 0 1 10 0v4" />
        </svg>
    );
}

function ViewIcon(props: SVGProps<SVGSVGElement>) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z" />
            <path d="M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" />
            <path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2" />
            <path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2" />
        </svg>
    );
}

function GoalIcon(props: SVGProps<SVGSVGElement>) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M12 13V2l8 4-8 4" />
            <path d="M20.55 10.23A9 9 0 1 1 8 4.94" />
            <path d="M8 10a5 5 0 1 0 8.9 2.02" />
        </svg>
    );
}
