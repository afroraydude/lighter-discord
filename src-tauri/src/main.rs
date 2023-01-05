#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowBuilder};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      WindowBuilder::new(
        app,
        "main".to_string(),
        tauri::WindowUrl::App("https://discord.com/app".into()),
      )
      .initialization_script(include_str!(
        "../js/litecordinject.js"
      ))
      .title("Lighter Cord")
      .min_inner_size(1280.0, 720.0)
      .build()?;


      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
