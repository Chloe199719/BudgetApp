"use client";
import { Provider } from "react-redux";
import { reduxStore } from "./store";
import useCheckUser from "@/components/auth/useCheckUser";
import { CurrentUserData } from "@/app/layout";

type Props = {
  userData?: CurrentUserData;
  children: React.ReactNode;
};

export const Providers = ({ children, userData }: Props) => {
  return (
    <Provider store={reduxStore}>
      <UserData userData={userData}>{children}</UserData>
    </Provider>
  );
};

export const UserData = ({ userData, children }: Props) => {
  useCheckUser({ userData });
  return <>{children}</>;
};
