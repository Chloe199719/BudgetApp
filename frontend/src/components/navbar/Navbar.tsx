"use client";

import { useSelector } from "@/lib/redux/store";

import AvatarComp from "./Avatar";
import Link from "next/link";
import { ModeToggle } from "./ThemeTogle";

type Props = {};
function Navbar({}: Props) {
  const auth = useSelector((state) => state.auth);

  return (
    <div className="flex w-full justify-between p-8">
      <h1 className="text-5xl text-pink-500 font-bold">
        <Link href="/">🐱 BudgetApp</Link>
      </h1>
      <div className="flex gap-2 items-center">
        {auth.isAuthenticated ? (
          <div className="flex gap-2 items-center">
            {auth.display_name} <AvatarComp />
          </div>
        ) : (
          <div className="flex gap-2 font-bold">
            <Link
              className="px-6 bg-blue-300 rounded-xl flex justify-center items-center"
              href={`/login`}
            >
              Sign-In
            </Link>
            <Link
              className="px-6 bg-blue-200 rounded-xl flex justify-center items-center"
              href={`/login`}
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
