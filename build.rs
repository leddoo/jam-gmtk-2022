fn main() {
    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        // FCK, I HATE TEXT. (no, you cannot use a single print)
        println!("cargo:rustc-link-arg=--js-library");
        println!("cargo:rustc-link-arg=D:/dev/jam-gmtk-2022/src/lib.js");
    }
}
