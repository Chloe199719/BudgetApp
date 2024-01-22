import { SignUpFormData } from "@/components/sign-up/SignupForm";
import axiosInstance from "../axios";

export async function register({
  email,
  password,
  unique_name,
  display_name,
}: SignUpFormData) {
  await axiosInstance.post("/register", {
    email,
    password,
    unique_name,
    display_name,
  });
}
