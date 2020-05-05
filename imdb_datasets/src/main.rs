fn main() {
	let now = std::time::Instant::now();
	println!("{:?}", now.elapsed().as_millis());
}