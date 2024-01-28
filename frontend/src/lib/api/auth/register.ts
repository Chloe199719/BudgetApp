import { SignUpFormData } from "@/components/auth/sign-up/SignupForm";
import axiosInstance from "../axios";
import { SuccessResponse } from "@/lib/types/sucesssResponse";
import { AxiosError } from "axios";
import { ErrorResponse } from "@/lib/types/errorResponse";

export async function registerUserApi({
    email,
    password,
    unique_name,
    display_name,
}: SignUpFormData) {
    try {
        const res = await axiosInstance.post("/users/register", {
            email,
            password,
            unique_name,
            display_name,
        });
        return res.data as SuccessResponse;
    } catch (error) {
        if (error instanceof AxiosError) {
            throw error.response?.data as ErrorResponse;
        }
        throw { error: "Unknown error" };
    }
}
