extern crate cc;

use std::env;

pub fn main() {
    let mut cc_handle = cc::Build::new();
    if env::var("TARGET").unwrap().contains("linux") {
        cc_handle.define("_GNU_SOURCE", None).file("src/linux/c/limits.c");
    }
    cc_handle.compile("liblimits.a");
}
