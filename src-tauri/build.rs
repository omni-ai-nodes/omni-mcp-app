fn main() {
    let _ = fix_path_env::fix(); // fix path env
    let args = std::env::args();
    let mut is_dev = false;
    for arg in args {
        if arg == "--dev" {
            is_dev = true;
            break;
        }
    }
    if is_dev {
        println!("cargo:rerun-if-changed=src-tauri");
        println!("cargo:rerun-if-changed=src-tauri/tauri.conf.json");
        println!("cargo:rerun-if-changed=src-tauri/src");
        println!("cargo:rerun-if-changed=src-tauri/src/main.rs");
        println!("cargo:rerun-if-changed=src-tauri/src/lib.rs");
        println!("cargo:rerun-if-changed=src-tauri/src/commands");
        println!("cargo:rerun-if-changed=src-tauri/src/commands/mod.rs");
        println!("cargo:rerun-if-changed=src-tauri/src/commands/mod.rs");
    }
    tauri_build::build()
}
