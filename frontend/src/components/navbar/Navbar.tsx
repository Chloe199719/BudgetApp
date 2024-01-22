'use client';

import { useSelector } from '@/lib/redux/store';

import AvatarComp, { defaultAvatar } from './Avatar';
import Link from 'next/link';
import { ModeToggle } from './ThemeTogle';
import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
} from '../ui/hover-card';
import { Avatar, AvatarFallback, AvatarImage } from '../ui/avatar';
import { CalendarIcon } from '@radix-ui/react-icons';
import { APP_NAME } from '@/lib/constants';

type Props = {};
function Navbar({}: Props) {
    const auth = useSelector((state) => state.auth);

    return (
        <div className="flex w-full justify-between p-8 bg-slate-900 dark:bg-black">
            <h1 className="text-5xl text-pink-500 dark:text-pink-900 font-bold hover:scale-95 active:scale-90">
                <Link href="/">🐱 {APP_NAME}</Link>
            </h1>
            <div className="flex gap-2 items-center">
                {auth.isAuthenticated ? (
                    <HoverCard>
                        <HoverCardTrigger>
                            <div className="flex gap-6 items-center">
                                <span className="text-xl">
                                    {auth.display_name}
                                </span>
                                <AvatarComp />
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
                                        <CalendarIcon className="mr-2 h-4 w-4 opacity-70" />{' '}
                                        <span className="text-xs text-muted-foreground">
                                            Joined on{' '}
                                            {new Date(
                                                auth.data_joined,
                                            ).toLocaleDateString('de-De')}
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
        </div>
    );
}
export default Navbar;
