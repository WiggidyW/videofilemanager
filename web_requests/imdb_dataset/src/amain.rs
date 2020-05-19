pub struct Foo {
	inner: String,
}

impl Foo {
	pub fn new() -> Self {
		Self { inner: "Foo".to_string() }
	}

	pub fn nil(self) -> Self {
		self
	}

	fn inner_mut(&mut self) -> &mut String {
		&mut self.inner
	}

	pub fn append(&mut self) -> &mut Self {
		self.inner_mut().push_str("Bar");
		self
	}
}

fn main() {
	let f = Foo::new()
		.nil()
		.append()
		.nil();
    println!("{}", &f.inner);
}
