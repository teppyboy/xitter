use tauri::{PhysicalSize, Size};

const INIT_SCRIPT: &str = include_str!("../bundle/script.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default().setup(|app| {
        let webview = tauri::WebviewWindowBuilder::new(
            app,
            "Xitter",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .initialization_script(INIT_SCRIPT)
        .resizable(false)
        .title("Xitter")
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 17_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/130.0.6723.78 Mobile/15E148 Safari/604.1")
        .build()?;
        // Modify the window size for system except mobiles.
        #[cfg(not(target_os = "android"))]
        {
            // We need to modify the first window size
            let monitor = app.primary_monitor().unwrap().unwrap();
            let monitor_size = monitor.size();
            // Big math to calculate the window size, only for systems that are not Android
            webview
                .set_size(Size::Physical(PhysicalSize {
                    // We want vertical window here
                    width: (monitor_size.height - 128) * 9 / 16,
                    height: monitor_size.height - 128,
                }))
                .unwrap();
        }
        Ok(())
    });
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
