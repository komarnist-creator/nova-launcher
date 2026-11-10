fn main() {
    // маленький C-модуль (папка native/)
    println!("cargo:rerun-if-changed=ram.c");
    cc::Build::new().file("ram.c").compile("nova_native");

    println!("cargo:rerun-if-changed=nova.ico");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("nova.ico");
        res.set("ProductName", "nova");
        res.set("FileDescription", "nova - Minecraft launcher");
        if let Err(e) = res.compile() {
            println!("cargo:warning=icon was not embedded: {}", e);
        }
    }
}
