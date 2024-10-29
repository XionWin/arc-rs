fn main() {
    // Search the linked library for macOS
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=all=/opt/local/lib")
    }
}
