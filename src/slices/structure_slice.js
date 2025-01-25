import { createSlice } from '@reduxjs/toolkit'

export const structureSlice = createSlice({
  name: 'structure',
  initialState: {
      sections: 0 
  },
  reducers: {
  }
})

export const { insert_section } = structureSlice.actions
export default structureSlice.reducer
