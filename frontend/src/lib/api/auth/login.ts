import { CurrentUserData } from "@/app/layout";
import axiosInstance from "../axios";
import { LoginFormType } from "@/components/auth/login/LoginForm";
import { AxiosError } from "axios";
import { ErrorResponse } from "@/lib/types/errorResponse";

export async function PostLogin(e: LoginFormType) {
    try {
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
    } catch (error) {
        if (error instanceof AxiosError) {
            throw error.response?.data as ErrorResponse;
        }
        throw { error: "Unknown error" };
    }
}
