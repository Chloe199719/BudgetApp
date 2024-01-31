import { AxiosError } from "axios";
import axiosInstance from "../axios";
import { CurrentUserData } from "@/app/layout";
import { ErrorResponse } from "@/lib/types/errorResponse";

export async function postEditProfile(
    formData: FormData,
): Promise<CurrentUserData> {
    try {
        const res = await axiosInstance.patch("/users/update_user", formData);
        const data = res.data as CurrentUserData;
        return data;
    } catch (error) {
        if (error instanceof AxiosError) {
            throw error.response?.data as ErrorResponse;
        }
        throw { error: "Unknown error" };
    }
}
