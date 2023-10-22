fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// depreciation
	let d = p * (1.0 - (r / 100.0)) * t;
	println!("Depreciation is {}", d);

}