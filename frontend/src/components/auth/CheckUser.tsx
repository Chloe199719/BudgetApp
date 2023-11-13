"use client";

import { CurrentUserData } from "@/app/layout";
import { login, logout } from "@/lib/redux/slices/auth";
import { useDispatch } from "@/lib/redux/store";
import { useMemo } from "react";

type Props = {
  userData?: CurrentUserData;
};
function CheckUser({ userData }: Props) {
  const dispatch = useDispatch();
  useMemo(() => {
    dispatch(
      userData ? login({ ...userData, isAuthenticated: true }) : logout()
    );
  }, [userData, dispatch]);
  return null;
}
export default CheckUser;
