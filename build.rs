fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Add platform-specific configuration
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cfg=feature=\"macos\"");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
} 