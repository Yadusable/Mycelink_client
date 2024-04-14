// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::convert::Infallible;
use mycelink_lib_api::api::APIConnector;
use mycelink_lib_api::db::db_connector::DBConnector;
use std::fmt::Debug;
use std::sync::{Mutex, MutexGuard};
use tokio::runtime::Runtime;
use tauri::State;
use serde::{Deserialize, Serialize};

mod cmd_endpoints;

#[derive(Clone, Deserialize)]
struct Contact{
    chat_id: String,
    nickname: String,
    profile_picture: String,
    last_message: String,
    status: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Chat {
    id: i64, // streichen
    display_name: Box<str>,
    alt_name: Option<Box<str>>,
    profile_picture: String,
    last_message: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct SelectedChat {
    chat: Chat,
}

struct AvailableChats {
    chats: Vec<Contact>
}

struct Message{
    sender_contact_id: String,
    sender_display_name: String,
    message_id: String,
    message_type: MessageTypes,
    timestamp: i64,
}

enum MessageTypes{
    Reply{
        root: String,
        inner: Box<MessageTypes>,
    }, Text {
        content: String
    }, Media {
        mime_type: String,
        media_size: String,
        media_id: String,
    }, Reaction {
        react_to: String,
        reaction: char,
    },
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(text: &str) -> String {
    format!(" Message: {} was received.", text)
}

#[tauri::command]
fn send_msg(text: &str, state: State<Mutex<Option<SelectedChat>>>) -> String {
    let nickname = state.lock().unwrap();
    match nickname.as_ref() {
        Some(nickname) => format!("Message, '{}' for  {} was received in the Backend!", text, nickname.chat.display_name),
        None => format!("no valid nickname provided")
    }
}

#[tauri::command]
fn select_chat(chat_details: State<Mutex<Option<SelectedChat>>>, ui_selected_chat: SelectedChat) {
    let mut chat_details = chat_details.lock().unwrap();
    *chat_details = Some(ui_selected_chat);
}

#[tauri::command]
fn get_selected_chat(chat_details: State<Mutex<Option<SelectedChat>>>) -> Option<SelectedChat> {
    chat_details.lock().unwrap().clone()
}

#[tauri::command]
async fn create_api_connector() -> Result<APIConnector<(), ()>, ()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        //APIConnector::new(DBConnector::new("./loremipsumdolorsitamat.sqlite".as_ref()).await.unwrap(), ) //TODO find a nice way to not depend on mycelink-lib-fcp
    });
    todo!()
}

fn main() {
    // todo Call init function from ml-lib
    tauri::Builder::default()
        .manage::<Mutex<Option<SelectedChat>>>(Mutex::new(None))
        // .manage::<>()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, send_msg, select_chat, get_selected_chat]) //, create_api_connector
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
