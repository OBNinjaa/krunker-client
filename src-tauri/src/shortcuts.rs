use tauri::{AppHandle, GlobalShortcutManager, Manager};

pub fn register(app_handle: AppHandle) {
    let mut shortcut_manager = app_handle.global_shortcut_manager();

    let window = app_handle.get_window("main").unwrap();
    if !shortcut_manager.is_registered("F11").unwrap() {
        shortcut_manager
            .register("F11", move || {
                if let Ok(is_fullscreen) = window.is_fullscreen() {
                    window.set_fullscreen(!is_fullscreen).unwrap();
                }
            })
            .unwrap();
    }

    let window = app_handle.get_window("main").unwrap();
    if !shortcut_manager.is_registered("F6").unwrap() {
        shortcut_manager
            .register("F6", move || {
                window
                    .eval("window.location.href = 'https://krunker.io/'")
                    .unwrap();
            })
            .unwrap();
    }

    let window = app_handle.get_window("main").unwrap();
    if !shortcut_manager.is_registered("F5").unwrap() {
        shortcut_manager
            .register("F5", move || {
                window.eval("window.location.reload()").unwrap();
            })
            .unwrap();
    }
}

pub fn unregister(app_handle: AppHandle) {
    app_handle
        .global_shortcut_manager()
        .unregister_all()
        .unwrap();
}