import axiosInstance from "../axios";
import { SuccessResponse } from "@/lib/types/sucesssResponse";
import { AxiosError } from "axios";
import { ErrorResponse } from "@/lib/types/errorResponse";
import { ForgotPasswordFormData } from "@/components/auth/forgotPassword/forgotPassword";

export async function forgotPasswordEmailRequest({
    email,
}: ForgotPasswordFormData) {
    try {
        const res = await axiosInstance.post("/users/request-password-change", {
            email,
        });
        return res.data as SuccessResponse;
    } catch (error) {
        if (error instanceof AxiosError) {
            throw error.response?.data as ErrorResponse;
        }
        throw { error: "Unknown error" };
    }
}
