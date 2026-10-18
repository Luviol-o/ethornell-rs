fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        // oboe-sys uses libc++_static; its ABI support must be linked too.
        println!("cargo:rustc-link-arg=-lc++abi");
    }
    let rev = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=ETHORNELL_SOURCE_REV={rev}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../platform/android/include/ethornell_android.h");
}
