//! Build script for Windows resources

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Link Windows libraries
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=shell32");

        // Embed Windows resources (icon, manifest, version info)
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/notepad++.ico")
            .set("ProductName", "Notepad++ Rust Edition")
            .set("FileDescription", "Notepad++ Text Editor (Rust)")
            .set("ProductVersion", "8.0.0")
            .set("FileVersion", "8.0.0");

        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
