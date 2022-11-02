fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // when a mutable reference is created, there can be no other references.
    // let mut s = String::from("hello");

    // let r1 = &mut s; 
    // let r2 = &s; // invalid

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("sequential mutable references work: {}", r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point, so scope ends (due to Non-Lexical Lifetimes)

    let r3 = &mut s; // no problem
    println!("simultaneous mut and other references, as long as non-mut aren't used after creation of mut: {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}