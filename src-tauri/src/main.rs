#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use winapi::{
    um::{
        winuser::{ GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId },
        processthreadsapi::OpenProcess, winbase::QueryFullProcessImageNameW, winnt::PROCESS_QUERY_LIMITED_INFORMATION,
        psapi::{ GetProcessImageFileNameW }
    },
    shared::minwindef::DWORD,
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct WindowInfo {
    platform: String,
    title: String,
    id: String,
    path: String,
    name: String
}

#[tauri::command]
fn get_foreground_window() -> String {
    unsafe {
        // Get the window handle
        let window_handle = GetForegroundWindow();

        // Get the process ID
        let mut process_id: DWORD = DWORD::default();
        let thread_id = GetWindowThreadProcessId(window_handle, &mut process_id);

        // Open a process handle to the window with limited access rights
        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id);

        // Get the window title with GetWindowTextW
        let window_title = {
            const MAX_TITLE_LENGTH: usize = 255;
            let mut buffer: [u16; MAX_TITLE_LENGTH] = [0; MAX_TITLE_LENGTH];
            let read_len = GetWindowTextW(window_handle, buffer.as_mut_ptr(), MAX_TITLE_LENGTH as i32);
            String::from_utf16_lossy(&buffer[0..read_len as usize])
        };

        // Get the window title with GetProcessImageFileNameW
        let process_path = {
            const MAX_LEN: usize = 1024;

            let mut buffer: [u16; MAX_LEN] = [0; MAX_LEN];
            let read_len = GetProcessImageFileNameW(process_handle, buffer.as_mut_ptr(), buffer.len() as u32);
            String::from_utf16_lossy(&buffer[0..read_len as usize])
        };

        let window_info = WindowInfo {
            platform: "windows".to_string(),
            title: window_title.to_string(),
            id: process_id.to_string(),
            path: process_path.to_string(),
            name: "".to_string()
        };

        println!("{:?}", window_info);

        let serialized_window_info = serde_json::to_string(&window_info).unwrap();

        return serialized_window_info;
    }
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_foreground_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
