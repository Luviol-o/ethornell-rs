// cfg aliases for the host file policy (src/file_policy.rs).
fn main() {
    println!("cargo::rustc-check-cfg=cfg(fd_policy)");
    println!("cargo::rustc-check-cfg=cfg(hosted_files)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let unix = std::env::var_os("CARGO_CFG_UNIX").is_some();
    // The consoles are `unix` to Rust but have no directory descriptors;
    // like the web, they hand the engine their file roots instead.
    let hosted = arch == "wasm32" || os == "vita" || os == "horizon";
    if hosted {
        println!("cargo::rustc-cfg=hosted_files");
    } else if unix {
        println!("cargo::rustc-cfg=fd_policy");
    }
}
