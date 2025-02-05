import { createSlice } from "@reduxjs/toolkit"

export const errorSlice = createSlice({
    name: "is_error",
    initialState: false,
    reducers: {
        set_error: (state) => {
            console.log("Set the error");
            return true;
        },
        remove_error: (state) => {
            return false;
        }
    }
})

export const { set_error, remove_error } = errorSlice.actions
export default errorSlice.reducer
