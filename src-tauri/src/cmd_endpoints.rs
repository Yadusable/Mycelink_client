use mycelink_lib_api::api::APIConnector;

#[tauri::command]
fn mycelink_health_check(api_connector: tauri::State<APIConnector<()>>) -> Result<(),()> {
    api_connector.health_check()
}