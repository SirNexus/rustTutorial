fn main() {
    let y = { // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');

    println!("Print the return value: {}", five());
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // Functions return the last expression implicitly.
      // Notice the lack of semicolon here, making 5 an expression rather than statement.
}