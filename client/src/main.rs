/*
 * What the Fuck am I Doing - track what the fuck you are doing all day
 *
 * Rust Edition
 *
 * (c) 2024 UW
 */
use windows::Win32::{
    Foundation::{HWND, MAX_PATH, POINT, CloseHandle},
    UI::{
        WindowsAndMessaging::{GetCursorPos, GetParent, GetWindowTextW, WindowFromPoint, GetWindowThreadProcessId},
        Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO}
    },
    System::{
        SystemInformation::{GetTickCount},
        ProcessStatus::{GetModuleFileNameExW},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}
    }
};

use serde::{Serialize, Deserialize};
use reqwest::blocking::{Client};

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use time::{OffsetDateTime, Date};

#[derive(Deserialize)]
struct Server {
    enable: bool,
    endpoint: String,
    secret: String,
    send_frequency: u32,
}

#[derive(Deserialize)]
struct Category {
    name: String,
    #[serde(with = "serde_regex", default)]
    window_title: Vec<Regex>,
    #[serde(with = "serde_regex", default)]
    process_path: Vec<Regex>
}

#[derive(Deserialize)]
struct Config {
    server: Server,
    user_inactive_threshold: u32,
    categories: Vec<Category>,
}

#[derive(Serialize)]
struct ActivityData {
    entries: HashMap::<String, HashMap::<String, u64>>
}

impl ActivityData {
    pub fn new() -> Self {
        ActivityData {
            entries: HashMap::new()
        }
    }
}

fn load_config() -> Config {
    let config_name = "waid.json";

    let config_path = Path::new(".").join(config_name);

    if !config_path.exists() {
        println!("Creating default config waid.json");

        fs::write(
            &config_path,
            r#"{
    "server": {
        "enable": true,
        "endpoint": "http://localhost:3010/api/activity",
        "secret": "placeholder",
        "send_frequency": 300
    },
    "user_inactive_threshold": 300,
    "categories": [{
        "name": "Steam",
        "process_path": ["^.*steamwebhelper.exe$"]
    }, {
        "name": "Gaming",
        "process_path": ["^.*steamapps\\\\common\\\\.*\\.exe$"]
    }, {
        "name": "Reddit",
        "window_title": ["^.*(Reddit)|(: r/).*$"]
    }, {
        "name": "YouTube",
        "window_title": ["^.*YouTube - Google Chrome$"]
    }, {
        "name": "Messenger",
        "window_title": ["^Messenger - Google Chrome$"]
    }, {
        "name": "Browsing",
        "window_title": ["^.* - Google Chrome$"]
    }, {
        "name": "Work",
        "window_title": ["^.*(Visual Studio Code$)|(Microsoft Visual Studio( \\(Administrator\\))?$)"]
    }, {
        "name": "Other",
        "window_title": ["^.*$"]
    }]
}"#,
        )
        .expect("Cannot create default config (insufficient permissions?)");
    }

    let config_file = fs::read_to_string(config_path)
        .expect("Cannot read config file (maybe the file doesn't exist?)");

    serde_json::from_str(&config_file).expect("File waid.json is invalid")
}

unsafe fn get_cursor_pos() -> POINT {
    let mut loc: POINT = POINT::default();

    let _ = GetCursorPos(&mut loc);

    return loc;
}

unsafe fn get_window_at(pos: POINT) -> (String, String) {
    let mut handle = WindowFromPoint(pos);
    let mut parent = GetParent(handle);

    while parent != HWND(0) {
        handle = parent;
        parent = GetParent(handle);
    }

    let mut title: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];

    GetWindowTextW(handle, &mut title);

    let mut title = String::from_utf16_lossy(&title);
    title.truncate(title.trim_matches(char::from(0)).len());

    let mut pid = 0;
    GetWindowThreadProcessId(handle, Some(&mut pid));

    let proc = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).unwrap();

    let mut path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];

    GetModuleFileNameExW(proc, None, &mut path);

    let mut path = String::from_utf16_lossy(&path);
    path.truncate(path.trim_matches(char::from(0)).len());

    CloseHandle(proc).unwrap();

    return (title, path);
}

fn get_window_category(title: &str, path: &str, config: &Config) -> String {
    for cat in &config.categories {
        for expr in &cat.window_title {
            if expr.is_match(title) {
                return String::from(&cat.name);
            }
        }

        for expr in &cat.process_path {
            if expr.is_match(path) {
                return String::from(&cat.name);
            }
        }
    }

    return String::from("Other");
}

fn format_date(date: Date) -> String {
    return format!("{}-{:0>2}-{:0>2}", date.year(), u8::from(date.month()), u8::from(date.day()));
}

unsafe fn get_user_last_active() -> u32 {
    let ticks = GetTickCount();

    // cbSize must be sizeof(LASTINPUTINFO)
    let mut last = LASTINPUTINFO::default();
    last.cbSize = std::mem::size_of::<LASTINPUTINFO>() as u32;

    let _ = GetLastInputInfo(&mut last);

    // GetTickCount() will overflow and become 0 after 49.7 days
    // Check if there was an overflow, and if so, correct the formula
    if ticks >= last.dwTime { ticks - last.dwTime } else { u32::MAX - last.dwTime + ticks }
}

fn send_data_to_server(client: &mut Client, data: &mut ActivityData, config: &Config) -> bool {
    if !config.server.enable || data.entries.len() == 0{
        return true;
    }

    println!("Sending data to server...");

    let res = client.post(&config.server.endpoint).header("X-Secret", &config.server.secret).json(&data).send();

    if let Ok(res) = res {
        if res.status().is_success() {
            data.entries.clear();
            return true;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = load_config();

    if args.iter().any(|i| i == "--setup") {
        // Only run the config setup
        return;
    }

    // { "yyyy-mm-dd": { "Category": time } }
    let mut data: ActivityData = ActivityData::new();

    let mut client = Client::new();

    unsafe {
        let mut last_activity = OffsetDateTime::now_local().unwrap();
        let mut last_request = last_activity;

        let mut last_request_ok = true;

        loop {
            let _ = Command::new("cmd")
                .args(["/c", "cls"])
                .status()
                .unwrap()
                .success();

            if last_request_ok {
                println!("WAID\n---");
            } else {
                println!("WAID (server error)\n---");
            }

            let (title, path) = get_window_at(get_cursor_pos());

            // TODO: PID cache; if the pid and title are the same, get the category from cache
            let category = get_window_category(&title, &path, &config);

            let now = OffsetDateTime::now_local().unwrap();

            let diff = now - last_activity;

            if diff.is_negative() {
                // Clock skew (e.g. daylight saving); ignore this iteration
                // This way, the clock continues smoothly and doesn't get confused by the hour change
                println!("Clock skew detected, ignoring");
            } else {
                // If the last iteration took >5s, most likely the OS went into sleep or hibernation
                if diff.as_seconds_f32() < 5.0 {
                    if now.date().day() != last_activity.date().day() {
                        // Day changed between iterations; ignore the time inbetween days
                        println!("Rise and shine, it's a new day!");
                    } else {
                        let user_last_active = get_user_last_active() / 1000;

                        if user_last_active >= config.user_inactive_threshold {
                            println!("INACTIVE");
                        } else {
                            println!("Title: {}\nPath: {}\nDetected: {}", title, path, category);

                            let date = format_date(now.date());

                            let today = &mut data.entries.entry(date).or_default();
                            *today.entry(category).or_insert(0) += diff.whole_milliseconds() as u64;

                            println!("User last active: {}s ago", user_last_active);
                        }
                    }
                }
            }

            last_activity = now;

            let request_diff = now - last_request;

            if request_diff.is_negative() {
                // Clock skew
                last_request = now;
            } else if request_diff.as_seconds_f32() >= config.server.send_frequency as f32 {
                last_request = now;
                last_request_ok = send_data_to_server(&mut client, &mut data, &config);
            }

            println!("---");

            for (date, collection) in &data.entries {
                println!("[{}]", &date);

                for (cat, time) in collection {
                    println!("{}: {}s", cat, time / 1000);
                }
            }

            sleep(std::time::Duration::from_millis(100));
        }
    }
}
