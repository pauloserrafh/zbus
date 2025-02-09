#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(data, zvariant::serialized::Context::<byteorder::LE>::new_gvariant(0));
    utils::fuzz_for_context(data, zvariant::serialized::Context::<byteorder::BE>::new_gvariant(0));
});
