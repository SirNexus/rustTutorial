use std::net::Ipv4Addr;


// main used to check for the differences between using the constructor
// for Ipv4Addr versus the parse method
fn main() {
	let ip_str = "127.0.0.1";

	// Ipv4Addr implements FromStr trait, which turns a string into
	// Ipv4Addr value so long as it follows the format
	let ip = ip_str.parse::<Ipv4Addr>().unwrap();
	
	// Call constructor
	let ip2 = Ipv4Addr::new(127, 0, 0, 1);	

	println!("IP Address: {:?}", ip);
	println!("IP Address: {}", ip2);

	// Check for equality
	if ip == ip2 {
		println!("They're equal!");
	}
}
