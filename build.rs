use std::env;
use std::fs;
use std::path::Path;

fn main() {
    copy_dlls().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn copy_dlls() -> std::io::Result<()> {
    // 获取输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    println!("out_dir: {}", out_dir);
    println!("out_path: {}", out_path.display());

    let steam_dll_source = Path::new("lib/steam_api64.dll");
    if steam_dll_source.exists() {
        let steam_dll_dest = out_path.join("steam_api64.dll");
        fs::copy(steam_dll_source, steam_dll_dest)?;
    }

    Ok(())
}
