#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::{sleep, self}, time::Duration};
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};

lazy_static! {
  static ref CLICKER_STATUS: Mutex<bool> = Mutex::new(false);
}

#[derive(Clone, serde::Serialize)]
struct Payload {}

#[tauri::command]
fn clicker_start(delay: i64, click_type: &str, click_count: i8) {
  println!("Starting clicker with delay: {}, click_type: {}, click_count: {}", delay, click_type, click_count);
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
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![clicker_start, clicker_stop, status_clicker])
    .setup(|app| {
      // match *CLICKER_STATUS.lock().unwrap() {
      //   true => app.emit_all("click:start", Payload{}),
      //   false => app.emit_all("click:stop", Payload{})
      // };
      
      // use app.emit_all("click:start", Payload{}); to emit to start a clicker
      // use app.emit_all("click:stop", Payload{}); to emit to stop a clicker
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
