import { PayloadAction, createSlice } from '@reduxjs/toolkit';

type PasswordMessages = {
    passwordResetSuccess: string | null;
    passwordResetFailure: string | null;
};

const initialState: PasswordMessages = {
    passwordResetSuccess: null,
    passwordResetFailure: null,
};

export const passwordsSlice = createSlice({
    name: 'passwords',
    initialState,
    reducers: {
        passwordResetSuccess: (state, action: PayloadAction<string>) => {
            state.passwordResetSuccess = action.payload;
            state.passwordResetFailure = null;
        },
        passwordResetFailure: (state, action: PayloadAction<string>) => {
            state.passwordResetSuccess = null;
            state.passwordResetFailure = action.payload;
        },
        reset(state) {
            state.passwordResetSuccess = null;
            state.passwordResetFailure = null;
        },
    },
});

export const { passwordResetSuccess, passwordResetFailure, reset } =
    passwordsSlice.actions;

export default passwordsSlice.reducer;
