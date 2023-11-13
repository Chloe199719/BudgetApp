"use client";

import { useSelector } from "@/lib/redux/store";
import Image from "next/image";

type Props = {};
function Navbar({}: Props) {
  const auth = useSelector((state) => state.auth);

  if (!auth.isAuthenticated) {
    return <div>Not Logged</div>;
  }
  return (
    <div className="flex w-full justify-between">
      <div>Chloe Discord Site</div>
      <div className="flex gap-2 items-center">
        {auth.display_name}{" "}
        {auth.profile.avatar_link && (
          <Image
            width={50}
            height={50}
            src={auth.profile.avatar_link!}
            alt="asd"
            priority
          />
        )}
      </div>
    </div>
  );
}
export default Navbar;
