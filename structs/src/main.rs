
struct Dog {
	breed: String,
	age: i32,
}

impl Dog {
	fn birthday(&mut self) {
		self.age += 1;
	}

	fn print_fact() {
		println!("Dog Fact!! Dogs only have sweat glands in their paws.");
	}
}

fn main() {
	let mut wiener = Dog {breed: String::from("Dachsund"), age: 4};  // Struct variable identifiers required

	wiener.birthday(); // Method requires wiener to be mutable

	println!("The dog's age is: {}", wiener.age);
	println!("The dog's breed is: {}", wiener.breed);

	Dog::print_fact(); // Associated functions don't require an instantiated variable
}
