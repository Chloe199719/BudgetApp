import { ConfirmPasswordFormType } from "@/components/auth/password/ResetPasswordForm";
import axiosInstance from "../axios";
import { SuccessResponse } from "@/lib/types/sucesssResponse";
import { AxiosError } from "axios";
import { ErrorResponse } from "@/lib/types/errorResponse";

export async function PostResetPassword({
    token,
    password,
}: ConfirmPasswordFormType & { token: string }) {
    try {
        const res = await axiosInstance.post("/users/change-user-password", {
            token,
            password,
        });

        return res.data as SuccessResponse;
    } catch (error) {
        if (error instanceof AxiosError) {
            throw error.response?.data as ErrorResponse;
        }
        throw { error: "Unknown error" };
    }
}
