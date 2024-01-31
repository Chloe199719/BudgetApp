"use client";
import { Button } from "@/components/ui/button";
import { SheetTrigger, SheetContent, Sheet } from "@/components/ui/sheet";
import Link from "next/link";
import {
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenu,
} from "@/components/ui/navigation-menu";
import { ModeToggle } from "./ThemeTogle";
import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
} from "../ui/hover-card";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { useSelector } from "@/lib/redux/store";
import AvatarComp, { defaultAvatar } from "./Avatar";
import { CalendarIcon } from "@radix-ui/react-icons";

export default function MainBar() {
    const auth = useSelector((state) => state.auth);

    return (
        <header className="flex h-20 w-full shrink-0 items-center px-4 md:px-6">
            <Sheet>
                <SheetTrigger asChild>
                    <Button className="lg:hidden" size="icon" variant="outline">
                        <MenuIcon className="h-6 w-6" />
                        <span className="sr-only">Toggle navigation menu</span>
                    </Button>
                </SheetTrigger>
                <SheetContent side="left">
                    <Link href="/">
                        <DollarSignIcon className="h-6 w-6" />
                        <span className="sr-only">Budget App</span>
                    </Link>
                    <div className="grid gap-2 py-6">
                        <Link
                            className="flex w-full items-center py-2 text-lg font-semibold"
                            href="/"
                        >
                            Home
                        </Link>
                        <Link
                            className="flex w-full items-center py-2 text-lg font-semibold"
                            href="#"
                        >
                            Budget
                        </Link>
                        <Link
                            className="flex w-full items-center py-2 text-lg font-semibold"
                            href="#"
                        >
                            Expenses
                        </Link>
                        <Link
                            className="flex w-full items-center py-2 text-lg font-semibold"
                            href="#"
                        >
                            Reports
                        </Link>
                    </div>
                </SheetContent>
            </Sheet>
            <Link className="mr-6 hidden lg:flex" href="/">
                <DollarSignIcon className="h-6 w-6" />
                <span className="sr-only">Budget App</span>
            </Link>
            <NavigationMenu className="hidden lg:flex">
                <NavigationMenuList>
                    <NavigationMenuLink asChild>
                        <Link
                            className="group inline-flex h-9 w-max items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-gray-100/50 data-[state=open]:bg-gray-100/50 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50 dark:data-[active]:bg-gray-800/50 dark:data-[state=open]:bg-gray-800/50"
                            href="/"
                        >
                            Home
                        </Link>
                    </NavigationMenuLink>
                    <NavigationMenuLink asChild>
                        <Link
                            className="group inline-flex h-9 w-max items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-gray-100/50 data-[state=open]:bg-gray-100/50 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50 dark:data-[active]:bg-gray-800/50 dark:data-[state=open]:bg-gray-800/50"
                            href="#"
                        >
                            Budget
                        </Link>
                    </NavigationMenuLink>
                    <NavigationMenuLink asChild>
                        <Link
                            className="group inline-flex h-9 w-max items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-gray-100/50 data-[state=open]:bg-gray-100/50 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50 dark:data-[active]:bg-gray-800/50 dark:data-[state=open]:bg-gray-800/50"
                            href="#"
                        >
                            Expenses
                        </Link>
                    </NavigationMenuLink>
                    <NavigationMenuLink asChild>
                        <Link
                            className="group inline-flex h-9 w-max items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-gray-100/50 data-[state=open]:bg-gray-100/50 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50 dark:data-[active]:bg-gray-800/50 dark:data-[state=open]:bg-gray-800/50"
                            href="#"
                        >
                            Reports
                        </Link>
                    </NavigationMenuLink>
                </NavigationMenuList>
            </NavigationMenu>
            <div className="ml-auto flex gap-2">
                {auth.isAuthenticated ? (
                    <HoverCard>
                        <HoverCardTrigger>
                            <div className="flex gap-6 items-center">
                                <span className="text-xl">
                                    {auth.display_name}
                                </span>
                                <AvatarComp
                                    avatar_link={auth.profile.avatar_link}
                                />
                            </div>
                        </HoverCardTrigger>
                        <HoverCardContent className="w-80">
                            <div className="flex justify-between space-x-4">
                                <Avatar>
                                    <AvatarImage
                                        src={
                                            auth.profile.avatar_link
                                                ? auth.profile.avatar_link
                                                : defaultAvatar
                                        }
                                    />
                                    <AvatarFallback>Chloe</AvatarFallback>
                                </Avatar>
                                <div className="space-y-1">
                                    <h4 className="text-sm font-semibold">
                                        @{auth.display_name}
                                    </h4>
                                    <p className="text-sm">
                                        This is suppose to be a bio
                                    </p>
                                    <div className="flex items-center pt-2">
                                        <CalendarIcon className="mr-2 h-4 w-4 opacity-70" />{" "}
                                        <span className="text-xs text-muted-foreground">
                                            Joined on{" "}
                                            {new Date(
                                                auth.data_joined,
                                            ).toLocaleDateString("de-De")}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </HoverCardContent>
                    </HoverCard>
                ) : (
                    <div className="flex gap-2 font-bold">
                        <Link
                            className="px-6 bg-blue-300 dark:bg-sky-800 hover:dark:bg-sky-600 hover:bg-blue-400 rounded-xl flex justify-center items-center active:translate-y-1"
                            href={`/auth/login`}
                        >
                            Sign-In
                        </Link>
                        <Link
                            className="px-6 bg-blue-200 dark:bg-sky-900 hover:bg-blue-400 dark:hover:bg-sky-600 rounded-xl flex justify-center items-center active:translate-y-1"
                            href={`/auth/register`}
                        >
                            Sign-Up
                        </Link>
                    </div>
                )}
                <ModeToggle />
            </div>
        </header>
    );
}

function DollarSignIcon(props: any) {
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
            <line x1="12" x2="12" y1="2" y2="22" />
            <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
        </svg>
    );
}

function MenuIcon(props: any) {
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
            <line x1="4" x2="20" y1="12" y2="12" />
            <line x1="4" x2="20" y1="6" y2="6" />
            <line x1="4" x2="20" y1="18" y2="18" />
        </svg>
    );
}
