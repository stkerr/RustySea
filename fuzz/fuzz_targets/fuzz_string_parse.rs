#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rusty_sea;
use rusty_sea::bigint::*;
use rusty_sea::bigint::utilities::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = create_bigint_from_string(s);
    }
});
