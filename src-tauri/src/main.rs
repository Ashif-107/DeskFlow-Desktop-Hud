// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};
#[cfg(target_os = "windows")] // gives us .hwnd()


mod db;
use db::{AppSession, init_db, save_session_to_db, get_category_summary_today,clear_app_usage_if_new_day};





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


#[cfg(target_os = "windows")]
use windows::{
    core::PCWSTR,
     Win32::{
        Foundation::{HWND, MAX_PATH, LPARAM},
        System::{
            ProcessStatus::K32GetModuleBaseNameW,
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_QUERY_LIMITED_INFORMATION},
        },
        UI::WindowsAndMessaging::*,
    },
};

#[cfg(target_os = "windows")]
fn get_active_window_info() -> Option<(String, String)> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return None;
        }

        // Get window title
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title);
        let title = String::from_utf16_lossy(&title[..len as usize]);

        // Get process ID
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // Open the process
        let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok()?;

        // Get the executable name
        let mut name = [0u16; MAX_PATH as usize];
        let len = K32GetModuleBaseNameW(process, None, &mut name) as usize;
        let process_name = String::from_utf16_lossy(&name[..len]);

        Some((title, process_name))
    }
}


#[tauri::command]
fn get_active_app() -> Option<(String, String)> {
    get_active_window_info()
}





#[cfg(target_os = "windows")]
#[tauri::command]
fn get_all_visible_windows() -> Vec<(String, String)> {
    let mut windows_info: Vec<(String, String)> = Vec::new();

        use windows::Win32::Foundation::BOOL; 

    unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let windows_info = &mut *(lparam.0 as *mut Vec<(String, String)>);

        // Skip invisible or empty title windows
        if !IsWindowVisible(hwnd).as_bool() || GetWindowTextLengthW(hwnd) == 0 {
            return  true.into();
        }

        // Get window title
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title);
        let title = String::from_utf16_lossy(&title[..len as usize]);

        // Skip system/UI windows
        let skip_titles = ["Program Manager", "Settings", "Windows Input Experience"];
        if skip_titles.iter().any(|t| title.contains(t)) {
            return true.into();
        }

        // Get process ID
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ, false, pid);
        let exe_name = if let Ok(process) = handle {
            let mut name = [0u16; 260];
            let len = K32GetModuleBaseNameW(process, None, &mut name);

            if len > 0 {
                String::from_utf16_lossy(&name[..len as usize])
            } else {
                // Fallback for UWP apps
                use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT};
                use windows::core::PWSTR;

                let mut full = [0u16; 260];
                let mut size = full.len() as u32;

                if QueryFullProcessImageNameW(process, PROCESS_NAME_FORMAT(0), PWSTR(full.as_mut_ptr()), &mut size).is_ok() {
                    String::from_utf16_lossy(&full[..size as usize])
                        .split('\\')
                        .last()
                        .unwrap_or("<unknown>")
                        .to_string()
                } else {
                    "<unknown>".to_string()
                }
            }
        } else {
            "<access denied>".to_string()
        };

        // Filter system/UWP background processes
        let skip_exe = [
            "SystemSettings.exe", "StartMenuExperienceHost.exe",
            "ShellExperienceHost.exe", "ApplicationFrameHost.exe",
            "TextInputHost.exe", "SearchApp.exe"
        ];

        if skip_exe.iter().any(|p| exe_name.eq_ignore_ascii_case(p)) {
            return true.into();
        }

        windows_info.push((title, exe_name));
        true.into()
 
    }

    unsafe {
        EnumWindows(Some(enum_window_proc), LPARAM(&mut windows_info as *mut _ as isize));
    }

    windows_info
}


//----- To Track the time period -------//

use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static LAST_APP: Lazy<Mutex<Option<(String, String, u64)>>> = Lazy::new(|| Mutex::new(None));

use std::collections::HashMap;

#[derive(Clone)]
struct RunningApp {
    start_time: u64,
    last_seen: u64,
}

static RUNNING_APPS: Lazy<Mutex<HashMap<(String, String), RunningApp>>> = Lazy::new(|| Mutex::new(HashMap::new()));


fn guess_category(title: &str, process: &str) -> String {
    let lowered = format!("{} {}", title.to_lowercase(), process.to_lowercase());

    if lowered.contains("spotify") {
        "Music".to_string()
    } else if lowered.contains("vscode") || lowered.contains("code") {
        "Work".to_string()
    } else if lowered.contains("chrome") || lowered.contains("brave") {
        if lowered.contains("youtube") || lowered.contains("netflix") {
            "Entertainment".to_string()
        } else if lowered.contains("docs") || lowered.contains("ChatGPT") || lowered.contains("slack") {
            "Work".to_string()
        } else if lowered.contains("github") || lowered.contains("gitlab") || lowered.contains("bitbucket") {
            "Work".to_string()
        } else if lowered.contains("research") || lowered.contains("papers") || lowered.contains("arxiv") {
            "Research".to_string()
        } else if lowered.contains("education") || lowered.contains("learning") || lowered.contains("courses") {
            "Education".to_string()
        }
        else {
            "Browsing".to_string()
        }
    } else if lowered.contains("game") {
        "Gaming".to_string()
    } else if lowered.contains("whatsapp") || lowered.contains("discord") || lowered.contains("teams") || lowered.contains("telegram") {
        "Chatting".to_string()
    } else {
        "Other".to_string()
    }
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_running_processes() -> Vec<String> {
    use windows::{
        Win32::{
            System::Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                TH32CS_SNAPPROCESS,
            },
            Foundation::HANDLE,
        },
    };

    let mut processes = Vec::new();

    unsafe {
        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();

        if snapshot.0 == -1 {
            return processes; // failed to get snapshot
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..len]);

                // Optional: Filter out system processes
                let skip = [
                    "svchost.exe",
                    "System Idle Process",
                    "System",
                    "winlogon.exe",
                    "csrss.exe",
                    "smss.exe",
                    "Registry",
                    "Idle",
                ];

                if !skip.iter().any(|&s| s.eq_ignore_ascii_case(&name)) {
                    processes.push(name.clone());
                }

                if !Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
    }

    processes
}
#[tauri::command]
fn get_category_summary() -> Result<HashMap<String, u64>, String> {
    get_category_summary_today().map_err(|e| e.to_string())
}





#[tauri::command]
fn store_score(date: String, score: f64) -> Result<(), String> {
    db::store_productivity_score(&date, score).map_err(|e| e.to_string())
}

















fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // ✅ Initialize the database

            init_db().expect("Failed to initialize DB");

            // ✅ Clear app_usage table if the date has changed
            clear_app_usage_if_new_day().expect("Failed to clear usage table on new day");

            
            
            let window = app
                .get_webview_window("main")
                .expect("`main` window not found");
            make_window_desktop_hud(&window);

        let app_handle = app.handle();
        tauri::async_runtime::spawn(async move {
        use tokio::time::{sleep, Duration};
        loop {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let visible_windows = get_all_visible_windows();

        let mut apps = RUNNING_APPS.lock().await;

        // Track seen this tick
        let mut seen_now = HashMap::new();

        for (title, process) in visible_windows {
            seen_now.insert((title.clone(), process.clone()), true);

            apps.entry((title.clone(), process.clone()))
                .and_modify(|entry| {
                    entry.last_seen = now;
                })
                .or_insert(RunningApp {
                    start_time: now,
                    last_seen: now,
                });
        }

        let flush_interval = 5;
        // Clean up apps that disappeared
        apps.retain(|(title, process), app| {
            let duration = now.saturating_sub(app.start_time);
            let inactive = app.last_seen < now;
            let should_flush = duration >= flush_interval;

            if inactive || should_flush {
        let session = AppSession {
            app_name: process.clone(),
            window_title: title.clone(),
            category: guess_category(title, process),
            start_time: app.start_time,
            end_time: if inactive { app.last_seen } else { now },
        };

        save_session_to_db(&session).unwrap();

        if should_flush && !inactive {
            // Update the start_time to now for the next flush
            app.start_time = now;
            app.last_seen = now;
            return true; // keep tracking
        }

        false // stop tracking if app disappeared
    } else {
        true // still tracking, nothing to do yet
    }
        });

        sleep(Duration::from_secs(1)).await;
             }
            });

            Ok(())
        })
        .invoke_handler(generate_handler![
                    init_position,
                    get_active_app,
                    get_all_visible_windows,
                    get_running_processes,
                    get_category_summary,
                    store_score                  

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        
}
