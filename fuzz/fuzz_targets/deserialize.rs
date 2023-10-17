#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate casper_wasm;
extern crate binaryen;

fuzz_target!(|data: &[u8]| {
	let binaryen_module = binaryen::tools::translate_to_fuzz(data);

	// enable binaryen's validation if in doubt.
	// assert!(binaryen_module.is_valid());

	let wasm = binaryen_module.write();

	let _module: casper_wasm::elements::Module = casper_wasm::deserialize_buffer(&wasm)
		.expect(
			"deserialize output of wasm-opt, indicating possible bug in deserializer",
		);
});
