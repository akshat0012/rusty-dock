import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  data: []
};

export const quickWindowDataSlice = createSlice({
  name: 'quick_window_data',
  initialState,
  reducers: {
    insert_data: (state, action) => {
      console.log("I am Starting")
      const newValues = action.payload;
      newValues.forEach(value => {
        if (!state.data.includes(value)) {
          state.data.push(value);
        }
      });
      console.log("I am Ending")
    }
  }
});

export const { insert_data } = quickWindowDataSlice.actions;

export default quickWindowDataSlice.reducer;

