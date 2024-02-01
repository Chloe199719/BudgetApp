"use client";
import { useSelector } from "@/lib/redux/store";
import Link from "next/link";

import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { defaultAvatar } from "../navbar/Avatar";
type Props = {};
function CurrentUserInformation({}: Props) {
    const auth = useSelector((state) => state.auth);
    if (!auth.isAuthenticated) return null;
    return (
        <section className="p-8 bg-white dark:bg-gray-900 rounded-lg shadow-md mb-8">
            <h2 className="text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100">
                Current User Information
            </h2>
            <div className="grid grid-cols-2 gap-6 text-gray-700 dark:text-gray-300">
                <div>
                    <p className="font-semibold">Display Name:</p>
                    <p>{auth.display_name}</p>
                </div>
                <div>
                    <p className="font-semibold">Avatar Image:</p>
                    <Avatar>
                        <AvatarImage
                            src={
                                auth.profile.avatar_link
                                    ? auth.profile.avatar_link
                                    : defaultAvatar
                            }
                        />
                        <AvatarFallback>User</AvatarFallback>
                    </Avatar>
                </div>
                <div>
                    <p className="font-semibold">Birthdate:</p>
                    <p>
                        {auth.profile.birth_date
                            ? new Date(
                                  auth.profile.birth_date,
                              ).toLocaleDateString("en-DE", {
                                  year: "numeric",
                                  month: "long",
                                  day: "numeric",
                              })
                            : "Not Provided"}
                    </p>
                </div>
                <div>
                    <p className="font-semibold">Phone Number:</p>
                    <p>{auth.profile.phone_number || "Not Provided"}</p>
                </div>
                <div className="col-span-2">
                    <p className="font-semibold">About Me:</p>
                    <p>{auth.profile.about_me || "Not Provided"}</p>
                </div>
                <div>
                    <p className="font-semibold">Pronouns:</p>
                    <p>{auth.profile.pronouns || "Not Provided"}</p>
                </div>
                <div>
                    <p className="font-semibold">GitHub Link:</p>
                    <Link
                        className="text-blue-500 hover:text-blue-700"
                        href={auth.profile.github_link || "#"}
                    >
                        {auth.profile.github_link || "Not Provided"}
                    </Link>
                </div>
            </div>
        </section>
    );
}
export default CurrentUserInformation;
