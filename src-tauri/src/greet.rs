// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greetings(name: &str) -> String {
    if name.is_empty() {
        return String::from("Please provide a name!");
    }
    format!("Hello, {}! You've been greeted from Rust!", name)
}