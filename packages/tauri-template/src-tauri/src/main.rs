#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic payload".to_string()
        };
        let loc = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column())).unwrap_or_default();
        let err_msg = format!("Panic in Velora application: {}\nLocation: {}\n", payload, loc);
        eprintln!("{}", err_msg);
        if let Ok(temp_dir) = std::env::var("TEMP") {
            let log_file = std::path::Path::new(&temp_dir).join("velora-crash.log");
            let _ = std::fs::write(log_file, &err_msg);
        }
    }));

    velora_app_lib::run();
}
