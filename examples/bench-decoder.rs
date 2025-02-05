extern crate casper_wasm;

#[cfg(feature = "std")]
fn rate(file_name: &'static str, iterations: u64) {
	use std::fs;

	use std::time::{Duration, Instant};

	let file_size = fs::metadata(file_name)
		.unwrap_or_else(|_| panic!("{} to exist", file_name))
		.len();
	let mut total = Duration::from_secs(0);

	for _ in 0..iterations {
		let start = Instant::now();
		let _module = casper_wasm::deserialize_file(file_name);
		let end = start.elapsed();

		total += end;
	}

	println!(
		"Rate for {}: {} MB/s",
		file_name,
		(file_size as f64 * iterations as f64 / (1024*1024) as f64) / // total work megabytes
		(total.as_millis() as f64 / 1000f64) // total seconds
	);
}

#[cfg(feature = "std")]
fn main() {
	rate("./res/cases/v1/clang.wasm", 10);
	rate("./res/cases/v1/hello.wasm", 100);
	rate("./res/cases/v1/with_names.wasm", 100);
}
#[cfg(not(feature = "std"))]
fn main() {
	panic!("Compilation requires --feature std")
}
