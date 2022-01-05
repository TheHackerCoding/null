#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tinkv::{self, Store};

fn main() {
  tauri::Builder::default()
    .manage()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
