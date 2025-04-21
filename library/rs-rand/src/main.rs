fn main() {
	use rand::prelude::*;

	if random() { // generates a boolean
		// Try printing a random unicode code point (probably a bad idea)!
		println!("char: {}", random::<char>());
	}

	let mut rng = thread_rng();
	let y: f64 = rng.gen(); // generates a float between 0 and 1

	println!("{}", y);
}
