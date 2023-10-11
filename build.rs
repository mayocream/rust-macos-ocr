fn main() {
    cc::Build::new()
        .file("./OCRBridge.m")
        .flag("-fobjc-arc") // Use Automatic Reference Counting
        .compile("OCRBridge");

    // 2. Emit flags to link against the frameworks when linking the final Rust binary
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Vision");
}
