use tauri::Size;
use tauri::Window;
use tauri::PhysicalSize;
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::UI::Shell::{SHAppBarMessage, APPBARDATA, ABM_NEW, ABM_SETPOS, ABE_TOP};
use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOOWNERZORDER};


#[tauri::command]
fn get_primary_monitor_info(window: Window) -> Option<(u32, u32)> {
    match window.primary_monitor() {
        Ok(Some(monitor)) => {
            let size = monitor.size();
            println!("Monitor size: {}x{}", size.width, size.height);
            return Some((size.width, size.height));
        },
        Ok(None) => {
            println!("No primary monitor found");
            return None;
        },
        Err(e) => {
            println!("Error getting monitor: {}", e);
            return None;
        }
    }
}

#[tauri::command]
fn set_window_size(window: Window, width: i32, height: i32) -> bool {
    match window.set_size(Size::Physical(PhysicalSize { width: width as u32, height: height as u32 })) {
        Ok(_) => {
            println!("Successfully Set the size of the bar");
        }
        Err(e) => {
            println!("ERROR:: window.set_size() \n{}", e);
            return false; // Return false on error
        }
    }

    let hwnd = match window.hwnd() {
        Ok(hwnd) => {
            println!("Handle of the Window is -> {:?}", hwnd);
            hwnd.0 as HWND
        }
        Err(e) => {
            eprintln!("Error getting HWND: {}", e);
            return false;
        }
    };
    let rectangle: RECT = RECT {
        left: 0,
        top: 0,
        right: 1920,
        bottom: height+20
    };
    let mut appbar_data: APPBARDATA = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        hWnd: hwnd,
        uCallbackMessage: 0,
        uEdge: ABE_TOP,
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
            SWP_NOACTIVATE | SWP_NOOWNERZORDER,
        );

    }

    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_primary_monitor_info, set_window_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
