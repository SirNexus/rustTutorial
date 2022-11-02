fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4"); // arm
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // arm
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // ..
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if number > 5 { 5 } else { number };

    println!("Max number is 5: {number}");

}