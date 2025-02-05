import { configureStore } from '@reduxjs/toolkit'
import errorReducer from './slices/error_slice'
import dockSettingsReducer from './slices/dock_settings_slice'

export default configureStore({
  reducer: {
    is_error: errorReducer,
    dock_settings: dockSettingsReducer
  }
})
