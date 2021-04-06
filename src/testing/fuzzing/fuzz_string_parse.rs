#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate bigint;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = bigint::create_bigint_from_string(&data)
    }
});
