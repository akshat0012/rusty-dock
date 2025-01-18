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

use super::config::read_config;
use super::hot_reload::hot_reload;

#[tauri::command]
pub fn talk_to_win_api(window: Window) -> bool {

    // read the config file
    match read_config(None) {
        Ok(current_config) => {
            println!("SUCCESS:: read_config(None)");
            println!("Theme is -> {}", current_config.theme);
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
    
            // TOP OR BOTTOM
            let mut top: i32 = 0;
            let mut left: i32 = 0;
            let mut right: i32 = 0;
            let mut bottom: i32 = 0;
            let mut u_edge: u32 = 0;

            if current_config.position == "top" {
                top = top+current_config.top_offset;
                bottom = current_config.height+current_config.top_offset;
                left = current_config.left_offset;
                right = screen_width - current_config.right_offset;
                u_edge = ABE_TOP;
            }
            else if current_config.position == "bottom" {
                top = screen_height - current_config.height - current_config.bottom_offset;
                bottom = screen_height-current_config.bottom_offset;
                left = current_config.left_offset;
                right = screen_width - current_config.right_offset;
                u_edge = ABE_BOTTOM;
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
                    rectangle.left,
                    rectangle.top,
                    rectangle.right - rectangle.left,
                    rectangle.bottom - rectangle.top,
                    SWP_NOACTIVATE
                );
            }
        }
        Err(e) => {
            println!("ERROR::read_config()\n{}", e);
        }
    }

    true
}
