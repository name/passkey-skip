#![windows_subsystem = "windows"]

use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, SetForegroundWindow, GetWindowTextW
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, KEYEVENTF_KEYUP, KEYBD_EVENT_FLAGS, VK_RETURN, VK_TAB
};
use windows::core::PCSTR;
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::HWND;
use systray::Application;
use clap::Parser;

const WINDOW_CLASS_NAME: &str = "Credential Dialog Xaml Host\0";
const WINDOW_TITLE_LEN: usize = 1024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    skip_pin: bool,
}

fn main() {
    let args = Args::parse();

    let mut app = Application::new().expect("Failed to create tray icon");
    app.set_tooltip("Passkey Skip").expect("Failed to set tooltip");
    app.set_icon_from_resource("app_icon").expect("Failed to set icon from resource");
    
    app.add_menu_item("Quit", |_| -> Result<(), std::io::Error> {
        std::process::exit(0);
    }).expect("Failed to add menu item");

    println!("Passkey Skip is running in the background");

    let mut last_processed_hwnd = HWND(0);
    let mut waiting_for_new_window = false;

    let window_thread = thread::spawn(move || {
        loop {
            if let Some(hwnd) = find_fido_prompt_window() {
                if is_passkey_window(hwnd) {
                    if hwnd.0 != last_processed_hwnd.0 {
                        if !waiting_for_new_window {
                            println!("Passkey window found!");
                            if args.skip_pin {
                                select_security_key_option_skip_pin(hwnd);
                            } else {
                                select_security_key_option(hwnd);
                            }
                            last_processed_hwnd = hwnd;
                            waiting_for_new_window = true;
                        }
                    } else if waiting_for_new_window && hwnd.0 != last_processed_hwnd.0 {
                        waiting_for_new_window = false;
                    }
                }
            } else {
                waiting_for_new_window = false;
                last_processed_hwnd = HWND(0);
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    app.wait_for_message();
    
    let _ = window_thread.join();
}

fn find_fido_prompt_window() -> Option<HWND> {
    unsafe {
        let hwnd = FindWindowA(PCSTR(WINDOW_CLASS_NAME.as_ptr()), None);
        if hwnd.0 != 0 {
            Some(hwnd)
        } else {
            None
        }
    }
}

fn is_passkey_window(hwnd: HWND) -> bool {
    unsafe {
        let mut title = vec![0u16; WINDOW_TITLE_LEN];
        let len = GetWindowTextW(hwnd, &mut title);
        if len == 0 {
            return false;
        }
        
        let title = String::from_utf16_lossy(&title[..len as usize]);
        title.contains("Windows Security")
    }
}

fn select_security_key_option(hwnd: HWND) {
    unsafe {
        SetForegroundWindow(hwnd);
        thread::sleep(Duration::from_millis(25));
        
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(25));

        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(25));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(25));

        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);

        println!("Security key selected and confirmed");
    }
}

fn select_security_key_option_skip_pin(hwnd: HWND) {
    unsafe {
        SetForegroundWindow(hwnd);
        thread::sleep(Duration::from_millis(500));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(3500));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_TAB.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);
        thread::sleep(Duration::from_millis(50));

        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(0), 0);
        keybd_event(VK_RETURN.0 as u8, 0, KEYBD_EVENT_FLAGS(KEYEVENTF_KEYUP.0), 0);

        println!("Security key selected with skip-pin mode");
    }
}
