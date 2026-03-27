fn main() {
    // Windows 链接配置
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }

    // 仅在 tauri-app feature 启用时调用
    #[cfg(feature = "tauri-app")]
    {
        tauri_build::build();
    }
}
