fn main() {
    add_to_startup();

    // Your existing program logic here...
}

fn add_to_startup() {
    #[cfg(target_os = "windows")]
    add_to_startup_windows();

    #[cfg(target_os = "linux")]
    add_to_startup_linux();

    #[cfg(target_os = "macos")]
    add_to_startup_macos();
}

#[cfg(target_os = "windows")]
fn add_to_startup_windows() {
    use winreg::enums::*;
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = std::env::current_exe().unwrap().to_string_lossy().into_owned();

    if let Ok(run) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE) {
        if let Err(e) = run.set_value("MyTypingTracker", &path) {
            eprintln!("Failed to add to startup: {}", e);
        }
    }
}

#[cfg(target_os = "linux")]
fn add_to_startup_linux() {
    use std::fs::File;
    use std::io::Write;
    let home_dir = std::env::var("HOME").unwrap();
    let desktop_entry = format!("{}/.config/autostart/my_typing_tracker.desktop", home_dir);
    let current_exe = std::env::current_exe().unwrap().to_str().unwrap().to_owned();

    let content = format!(
        "[Desktop Entry]\nType=Application\nExec={}\nHidden=false\nX-GNOME-Autostart-enabled=true\nName=My Typing Tracker",
        current_exe
    );

    if let Err(e) = File::create(desktop_entry).and_then(|mut f| f.write_all(content.as_bytes())) {
        eprintln!("Failed to add to startup: {}", e);
    }
}

#[cfg(target_os = "macos")]
fn add_to_startup_macos() {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    let app_path = std::env::current_exe().unwrap();
    let app_path_str = app_path.to_str().unwrap();

    // AppleScript to add the application to login items
    let script_content = format!(r#"
tell application "System Events" to make new login item at end with properties {{path:"{}", hidden:false}}
"#, app_path_str);

    // Write the AppleScript to a temporary file
    let script_path = "/tmp/add_to_startup.scpt";
    let mut file = match File::create(script_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create AppleScript file: {}", e);
            return;
        },
    };

    if let Err(e) = file.write_all(script_content.as_bytes()) {
        eprintln!("Failed to write to AppleScript file: {}", e);
        return;
    }

    // Execute the AppleScript
    let output = Command::new("osascript")
        .arg(script_path)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("AppleScript execution failed");
            }
        },
        Err(e) => eprintln!("Failed to execute AppleScript: {}", e),
    }
}

