// Naming convention: All caps SNAKE_CASE
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constants must be immutable,
                                                 // and annotated with a type.

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let mut x = 5;

    x = x + 1;

    {
        x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    test_tuple();
    test_array();
}

fn test_tuple() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2; 

    println!("first value: {five_hundred}\nsecond value: {six_point_four}\nthird value: {one}");

    let _x = (); // A unit represents an empty value
}

fn test_array() {
    let a = [1, 2, 3, 4, 5];

    println!("array a: {:?}", a);

    let a = [3; 5];

    println!("arrays can have duplicate elems when initialized: {:?}", a);
}