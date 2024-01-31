import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export type AuthState = NotLoggedIn | LoggedIn;
type NotLoggedIn = {
    isAuthenticated: false;
};
type LoggedIn = {
    isAuthenticated: true;
    id: string;
    email: string;
    display_name: string;
    unique_name: string;
    is_active: boolean;
    is_staff: boolean;
    is_superuser: boolean;
    thumbnail?: string | null;
    data_joined: string;
    profile: {
        id: string;
        phone_number?: string | null;
        about_me?: string | null;
        pronouns?: string | null;
        avatar_link?: string | null;
        birth_date?: string | null;
        github_link?: string | null;
    };
};

const initialState: AuthState = {
    isAuthenticated: false,
} as AuthState;

export const authSlice = createSlice({
    name: "auth",
    initialState,
    reducers: {
        login: (state, action: PayloadAction<LoggedIn>) => {
            state = { ...action.payload } as LoggedIn;

            return state;
        },
        logout: (state) => {
            state = initialState;
            return state;
        },
    },
});

export const { login, logout } = authSlice.actions;
export default authSlice.reducer;
