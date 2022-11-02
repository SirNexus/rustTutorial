fn main() {
    let mut counter = 0;

    let result = loop { // loop is an expression when used with a break expression.
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks can include an expression, which is returned
        }
    };

    println!("The result is {result}"); // result is the value set by the break expression.

    nested_loop();
    while_loop();
    for_iter();
}

fn nested_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // you can break out of outer loops using labels.
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 0;
    while number < 3 {
        println!("number = {number}");
        number += 1;
    }
}

fn for_iter() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("element = {element}");
    }

    for number in (1..4).rev() {
        println!("number = {number}");
    }
}