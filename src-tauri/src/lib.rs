const INJECTED_JS: &str = include_str!("injected.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|window, _payload| {
            // window.open_devtools(); // force open
            window.eval(INJECTED_JS).unwrap();
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
