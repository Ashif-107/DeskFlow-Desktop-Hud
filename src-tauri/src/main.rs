// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};
#[cfg(target_os = "windows")] // gives us .hwnd()

#[tauri::command]
fn init_position(window: tauri::Window) {
    // place the window bottom‑right on the primary monitor
    if let Some(monitor) = window.current_monitor().unwrap() {
        let size = monitor.size();

        let width = 500.0;
        let height = 400.0;
        let x_margin = 24.0;
        let y_margin = 84.0;

        let x = size.width as f64 - width - x_margin;
        let y = size.height as f64 - height - y_margin;

        let _ = window.set_size(PhysicalSize::new(width, height));
        let _ = window.set_position(PhysicalPosition::new(x, y));
    }
}

// --------------------------------------------------------------------------
// Windows‑specific glue to attach the Tauri window to the *desktop* layer so
// it behaves like a Rainmeter skin: covered by normal apps but still visible
// after Win + D (Show Desktop).
// --------------------------------------------------------------------------
#[cfg(target_os = "windows")]
mod win {
    use windows::{
        core::PCSTR,
        Win32::{
            Foundation::HWND,
            UI::WindowsAndMessaging::{
                FindWindowExA, SetWindowLongPtrW, SetWindowPos, GWLP_HWNDPARENT, HWND_BOTTOM,
                SWP_NOMOVE, SWP_NOSIZE,
            },
        },
    };

    pub unsafe fn attach_to_desktop(hwnd: HWND) {
        // locate the desktop’s SHELLDLL_DefView window
        let progman = FindWindowExA(HWND(0), HWND(0), PCSTR(b"Progman\0".as_ptr()), PCSTR::null());
        let mut def_view =
            FindWindowExA(progman, HWND(0), PCSTR(b"SHELLDLL_DefView\0".as_ptr()), PCSTR::null());

        // multi‑monitor setups sometimes stuff it inside a WorkerW window
        if def_view.0 == 0 {
            let workerw =
                FindWindowExA(HWND(0), HWND(0), PCSTR(b"WorkerW\0".as_ptr()), PCSTR::null());
            def_view = FindWindowExA(
                workerw,
                HWND(0),
                PCSTR(b"SHELLDLL_DefView\0".as_ptr()),
                PCSTR::null(),
            );
        }

        // make the desktop the *owner* of our window so it rides that layer
        if def_view.0 != 0 {
            SetWindowLongPtrW(hwnd, GWLP_HWNDPARENT, def_view.0 as isize);
        }

        // push to the very bottom so normal windows cover it
        SetWindowPos(
            hwnd,
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        );
    }
}

fn make_window_desktop_hud(window: &WebviewWindow) {
     #[cfg(target_os = "windows")]
    if let Ok(raw_hwnd) = window.hwnd() {
        unsafe {
            // convert tauri's HWND to windows::Win32::Foundation::HWND
            let hwnd = windows::Win32::Foundation::HWND(raw_hwnd.0 as isize);

            win::attach_to_desktop(hwnd);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("`main` window not found");
            make_window_desktop_hud(&window);
            Ok(())
        })
        .invoke_handler(generate_handler![init_position])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
