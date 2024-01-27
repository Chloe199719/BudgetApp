import { CurrentUserData } from "@/app/layout";
import axiosInstance from "../axios";
import { LoginFormType } from "@/components/login/LoginForm";

export async function PostLogin(e: LoginFormType) {
    const res = await axiosInstance.post(
        "/users/login/",
        {
            email: e.email,
            password: e.password,
        },
        { withCredentials: true },
    );
    const data = res.data as CurrentUserData;
    return data;
}
