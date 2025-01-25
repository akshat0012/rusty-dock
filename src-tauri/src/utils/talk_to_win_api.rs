use tauri::Window;
use windows_sys::Win32::Foundation::{
    HWND, 
    RECT
};
use windows_sys::Win32::UI::Shell::{
    SHAppBarMessage, 
    APPBARDATA, 
    ABM_NEW, 
    ABM_SETPOS, 
    ABE_TOP,
    ABE_BOTTOM
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, 
    SWP_NOACTIVATE, 
    SM_CYSCREEN, 
    SM_CXSCREEN,
    SetWindowPos, 
    GetSystemMetrics
};

use super::config::Config;

pub fn talk_to_win_api(window: &Window, current_config: &Config) -> bool {
            let (screen_height, screen_width) = unsafe {
                (GetSystemMetrics(SM_CYSCREEN), GetSystemMetrics(SM_CXSCREEN))
            };

            let hwnd = match window.hwnd() {
                Ok(hwnd) => {
                    hwnd.0 as HWND
                }
                Err(e) => {
                    eprintln!("Error getting HWND: {}", e);
                    return false;
                }
            };

            let mut top: i32 = 0;
            let mut left: i32 = 0;
            let mut right: i32 = 0;
            let mut bottom: i32 = 0;
            let mut u_edge: u32 = 0;
    
            let mut y = 0;
            let mut x = 0;
            let mut cx = 0;
            let mut cy = 0;


            if current_config.dock_settings.position == "top" {
                top    = current_config.dock_settings.top_padding;
                bottom = current_config.dock_settings.height + current_config.dock_settings.top_padding + current_config.dock_settings.bottom_padding;
                right  = screen_width          - current_config.dock_settings.right_padding;
                left   = current_config.dock_settings.left_padding;
                u_edge = ABE_TOP;
                
                x = left;
                y = top;
                cx = right - left;
                cy = bottom - top - current_config.dock_settings.bottom_padding; 
            }

            else if current_config.dock_settings.position == "bottom" {
                bottom = screen_height - current_config.dock_settings.bottom_padding;
                top    = bottom - current_config.dock_settings.height - current_config.dock_settings.top_padding;
                right  = screen_width - current_config.dock_settings.right_padding;
                left   = current_config.dock_settings.left_padding;
                u_edge = ABE_BOTTOM;

                x = left;
                y = top + current_config.dock_settings.top_padding;
                cx = right - left;
                cy = bottom - top - current_config.dock_settings.top_padding;
            }


            let rectangle: RECT = RECT {
                left,
                right,
                bottom,
                top
            };
            let mut appbar_data: APPBARDATA = APPBARDATA {
                cbSize: std::mem::size_of::<APPBARDATA>() as u32,
                hWnd: hwnd,
                uCallbackMessage: 0,
                uEdge: u_edge,
                rc: rectangle,
                lParam: 0
            };
                
            unsafe {
                SHAppBarMessage(ABM_NEW, &mut appbar_data);
                SHAppBarMessage(ABM_SETPOS, &mut appbar_data);

                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    x,
                    y,
                    cx,
                    cy,
                    SWP_NOACTIVATE
                );
            }
    true
}
