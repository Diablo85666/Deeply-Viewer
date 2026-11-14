#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| deeply_fuzz::run("packet", data));
