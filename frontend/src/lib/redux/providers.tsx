"use client";
import { Provider } from "react-redux";
import { reduxStore } from "./store";
import useCheckUser from "@/components/auth/useCheckUser";
import { CurrentUserData } from "@/app/layout";
import { QueryClient, QueryClientProvider } from "react-query";
type Props = {
    userData?: CurrentUserData;
    children: React.ReactNode;
};
const queryClient = new QueryClient();

export const Providers = ({ children, userData }: Props) => {
    return (
        <Provider store={reduxStore}>
            <QueryClientProvider client={queryClient}>
                <UserData userData={userData}>{children}</UserData>
            </QueryClientProvider>
        </Provider>
    );
};

export const UserData = ({ userData, children }: Props) => {
    useCheckUser({ userData });
    return <>{children}</>;
};
