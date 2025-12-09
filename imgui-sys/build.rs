#![allow(dead_code)]

// Output define args for compiler
fn main() -> std::io::Result<()> {
    // Root of imgui-sys
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!(
        "cargo:THIRD_PARTY={}",
        manifest_dir.join("third-party").display()
    );

    println!("cargo:THIRD_PARTY=third-party/imgui-master");
    println!("cargo::rustc-link-lib=bnusio");

    Ok(())
}
