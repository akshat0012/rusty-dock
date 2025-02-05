import { createSlice } from '@reduxjs/toolkit'

export const dockSettingsSlice = createSlice({
  name: 'dock_settings',
  initialState: {
    bg_color: '#181818',
    border_radius: 10 
  },
  reducers: {
    set_radius: ( state, action ) => {
       state.border_radius = action.payload; 
    },
    set_color: ( state, action ) => {
        state.bg_color = action.payload;
    }
  }
})

export const { set_radius, set_color } = dockSettingsSlice.actions
export default dockSettingsSlice.reducer
