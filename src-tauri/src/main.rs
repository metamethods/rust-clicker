#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use inputbot::{KeySequence, KeybdKey::{*, self}, MouseButton::*};
use std::{thread::{sleep, self}, time::Duration};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
  static ref CLICKER_STATUS: Mutex<bool> = Mutex::new(false);
}

#[tauri::command]
fn clicker_start(delay: i64, hotkey: u64, click_type: &str, click_count: i8) {
  println!("Starting clicker with delay: {}, hotkey: {}, click_type: {}, click_count: {}", delay, hotkey, click_type, click_count);
  *CLICKER_STATUS.lock().unwrap() = true;
}

#[tauri::command]
fn clicker_stop() {
  println!("Stopping clicker");
  *CLICKER_STATUS.lock().unwrap() = false;
}

#[tauri::command]
fn status_clicker() -> bool {
  return *CLICKER_STATUS.lock().unwrap();
}

fn main() {
  // Start detecting input events
  thread::spawn(|| inputbot::handle_input_events());

  // Launch the app
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![clicker_start, clicker_stop, status_clicker])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
