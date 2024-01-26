import authSlice from "./slices/auth";
import passwordsSlice from "./slices/forgotPasswordSlice/passwords";
export const reducer = {
    auth: authSlice,
    passwords: passwordsSlice,
};
