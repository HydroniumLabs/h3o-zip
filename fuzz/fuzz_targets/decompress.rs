#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    h3o_zip::decompress(data).collect::<Result<Vec<_>, _>>();
});
