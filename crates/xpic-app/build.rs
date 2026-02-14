fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=locales");
    println!("cargo:rerun-if-changed=assets");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_icon("assets/app-icon.ico")
            .compile()
            .unwrap();
    }
}
