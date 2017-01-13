extern crate libbindgen;

// use std::env;
use std::path::Path;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    let out = Path::new("src").join("weechat-plugin.rs");

    // Only generate if it's not there. The reason being I have
    // to modify it to compile.
    if !out.exists() {
        let _ = libbindgen::builder()
            .header("/usr/include/weechat/weechat-plugin.h")
            .no_unstable_rust()
            .generate()
            .unwrap()
            .write_to_file(out);
    }
}