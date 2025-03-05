mod command;
use tauri::menu::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|handle| {
            let menu = Menu::with_items(
                handle,
                &[
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "Edit",
                        true,
                        &[
                            &PredefinedMenuItem::undo(handle, None)?,
                            &PredefinedMenuItem::redo(handle, None)?,
                            &PredefinedMenuItem::cut(handle, None)?,
                            &PredefinedMenuItem::copy(handle, None)?,
                            &PredefinedMenuItem::paste(handle, None)?,
                            &PredefinedMenuItem::select_all(handle, None)?,
                        ],
                    )?,
                ],
            );
            menu
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            command::cmds::open_window,
            command::cmds::preview_from_config,
            command::cmds::update_build_file,
            command::cmds::update_config_file,
            command::cmds::update_cargo_file,
            command::cmds::update_main_rust,
            command::cmds::rust_lib_window,
            command::cmds::update_custom_js,
            command::cmds::content_to_base64,
            command::cmds::update_config_json,
            command::cmds::rust_main_window,
            command::cmds::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
