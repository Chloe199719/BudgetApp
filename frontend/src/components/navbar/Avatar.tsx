import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import logout from "@/lib/api/auth/logout";
import { logout as reduxLogout } from "@/lib/redux/slices/auth";
import { useDispatch } from "@/lib/redux/store";
import { useToast } from "../ui/use-toast";
import Link from "next/link";

type Props = {
    avatar_link?: string | null;
};
export const defaultAvatar =
    "https://chloepratas-discordcopy.s3.eu-central-1.amazonaws.com/media/discord_backend/avatar/a6c0c3ac-31de-4daa-a569-19a3a50fccee/cutechleo.jpg";
function AvatarComp({ avatar_link }: Props) {
    const dispatch = useDispatch();
    const { toast } = useToast();
    return (
        <DropdownMenu>
            <DropdownMenuTrigger>
                {" "}
                <Avatar>
                    <AvatarImage
                        src={avatar_link ? avatar_link : defaultAvatar}
                    />
                    <AvatarFallback>User</AvatarFallback>
                </Avatar>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuLabel>
                    {" "}
                    <Link href={`/user/profile`}>My Account </Link>
                </DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuItem>Profile</DropdownMenuItem>
                <DropdownMenuItem>Billing</DropdownMenuItem>
                <DropdownMenuItem>Team</DropdownMenuItem>
                <DropdownMenuItem
                    onClick={async () => {
                        try {
                            await logout();
                            dispatch(reduxLogout());
                            toast({
                                title: "Logged out",
                                description: "You have been logged out",
                            });
                        } catch (error) {
                            console.log(error);
                        }
                    }}
                >
                    Logout
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    );
}
export default AvatarComp;
