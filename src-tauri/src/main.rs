// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use mycelink_lib_api::api::APIConnector;
use mycelink_lib_api::db::db_connector::DBConnector;
use tauri::async_runtime::block_on;
use tokio::runtime::Runtime;

mod cmd_endpoints;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(create_api_connector())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn create_api_connector() -> APIConnector<()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async{
        APIConnector::<()>::new(DBConnector::new("./loremipsumdolorsitamat.sqlite".as_ref()).await.unwrap())
    })
}