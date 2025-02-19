import { configureStore } from '@reduxjs/toolkit'
import errorReducer from './slices/error_slice'
import dockSettingsReducer from './slices/dock_settings_slice'
import quickWindowDataReducer from './slices/quick_window_data_slice'

export default configureStore({
  reducer: {
    is_error: errorReducer,
    dock_settings: dockSettingsReducer,
    quick_window_data: quickWindowDataReducer
  }
})
