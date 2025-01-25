import { configureStore } from '@reduxjs/toolkit'
import dockSettingsReducer from './slices/dock_settings'

export default configureStore({
  reducer: {
    dock_settings: dockSettingsReducer 
  }
})
