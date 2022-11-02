fn main() {
    let len = first_word(&String::from("hello world"));
    println!("size of first word: {}", len);

    // We need to declare the var here, since it owns the memory that we're
    // referencing below by creating the reference.
    let str = String::from("hello world");

    let s = first_word_slice(&str);
    println!("size of first word: {}", s);

    let s = "Hello, world!";
    println!("string literals are slices: {s}");
}

// The problem with this function, is that we're returning a usize that doesn't
// have any inherent connection to the passed in string. This means that if the
// string itself goes out of scope, the usize will still stick around but not
// be meaningful for anything. The goal should be to have a value that has some
// sort of connection to the context with which it was created.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // the returned reference, "item", is a reference. If we declare as
    // "item", we'll have the type be &u8. We could dereference with *item below, but
    // specifying "&item" means we don't have to dereference.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// str is a slice type. Must be a reference, since return type sizes must be known
// at compile time.
// note the &str in the parameter list, which can take a String or string slice,
// due to deref coercions
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}