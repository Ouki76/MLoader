#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;

mod modules;

#[tokio::main]
async fn main() {
    start().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            modules::cheat::parser::get_repos_json,
            modules::cheat::lua::run_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn start() {
    std::fs::create_dir_all(format!(
        "{}{}",
        modules::loader::settings::PATH,
        "repositories"
    ))
    .unwrap();

    modules::loader::settings::create().await;

    std::fs::File::create(format!("{}injector.dll", modules::loader::settings::PATH))
        .unwrap()
        .write(include_bytes!("../../BlackBone/build/Release/injector.dll"))
        .unwrap();
}
