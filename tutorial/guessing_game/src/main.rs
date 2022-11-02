// Prelude
use std::io; // Similar to C++ using
use rand::Rng; // Rng (trait for ThreadRng) must be in scope to use gen_rance
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    let secret_number=  rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Q: Why does this need to be in a loop?
        let mut guess = String::new(); // mut means mutable
                                   // :: means associated function with a type
                                   // new() often refers to an empty version of that type

        io::stdin()
            .read_line(&mut guess) // must be mutable so that read_line can change it
                                // returns a `Result` enum, where is state is a "variant"
            .expect("Failed to read line");
        
        // Rust allows variable shadowing, even for different types
        let guess_num: u32 // creating a u32 type and comparing below automatically infers the 
                    // secret_number above should be a u32 as well.
            = match guess.trim() 
              .parse() { // we can parse into any type implementing the `FromStr` trait
                  Ok(t) => t,
                  Err(_) => continue,
              };

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    
        println!("You guessed: {guess}");
    }

}
