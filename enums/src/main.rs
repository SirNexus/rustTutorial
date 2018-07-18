
// Enums have a constructor with optional type
#[derive(Debug)]
enum Message {
	Quit,					// Only constructor, no type
	Text(String),			// Constructor with type String
	Move { x: i32, y: i32}, // Custom struct definition inside enum
}

impl Message {
	// TODO: Implement Display method for Message
}

fn main() {
	// Initialize variables with <enum>::<value> syntax
	let x = Message::Text(String::from("Your dog is as smart as a 2-year-old toddler."));

	println!("Dog Fact!! {:?}", x);

}
