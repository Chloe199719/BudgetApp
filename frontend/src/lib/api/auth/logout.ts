import axiosInstance from "../axios";

export default async function logout() {
  return await axiosInstance.post("/users/logout/");
}
