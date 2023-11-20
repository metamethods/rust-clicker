#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use device_query::{DeviceQuery, DeviceState, Keycode};

use enigo::{Enigo, MouseButton, MouseControllable, KeyboardControllable};

lazy_static! {
    static ref CLICKER_STATUS: Mutex<bool> = Mutex::new(false);
}


#[derive(Clone, serde::Serialize)]
struct Payload {}

#[tauri::command]
fn clicker_start(delay: i64, click_type: String, click_count: i8) {
    println!(
        "Starting clicker with delay: {}, click_type: {}, click_count: {}",
        delay, click_type, click_count
    );
    *CLICKER_STATUS.lock().unwrap() = true;
    thread::spawn(move || {
      let mut enigo = Enigo::new();
  
      loop {
        if !status_clicker() { 
          break;
        }
  
        if status_clicker() {
          enigo.mouse_click(convert_mouse_str(&click_type));
          
          thread::sleep(Duration::from_millis(delay as u64));
        }
      }
    });
}

fn convert_mouse_str(click_type: &str) -> MouseButton {

  match click_type {
    "Left" => MouseButton::Left,
    "Right" => MouseButton::Right, 
    "Middle" => MouseButton::Middle,
    _ => MouseButton::Left,
  }
}

#[tauri::command]
fn clicker_stop() {
    println!("Stopping clicker");
    *CLICKER_STATUS.lock().unwrap() = false;
}

#[tauri::command]
fn status_clicker() -> bool {
    *CLICKER_STATUS.lock().unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![clicker_start, clicker_stop, status_clicker])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
