fn main() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();

    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // passing variable to a function gives the ownership to that function and variable gets cleared after
    // that's why we can't use it after the function call
    let s4 = String::from("hello");
    take_ownership(s4);
    // println!("{}", s4); // ownership taken

    // simple values get copied
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // borrowing a value
    let s = String::from("hello");
    let (s, length) = calculate_length(s);
    println!("Length of {} is {}", s, length);

    // passing a reference
    let s = String::from("hello");
    let length = calculate_length2(&s);
    println!("Length of {} is {}", s, length);

    // passing a mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("New value is {}", s);

    // mutable and non mutable references
    // we can have multiple non mutable references or one mutable reference in a single scope
    // after the last tile ref1 and ref2 are used, the scope of non mutable references to s is over and so we can define a mut ref to s at that pointd
    let mut s = String::from("hello");

    let ref1 = &s;
    let ref2 = &s;

    println!("{}, {}", ref1, ref2);

    let ref3 = &mut s;
    ref3.push_str(" world.");
    println!("{}", ref3);

    // string slices
    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let s2 = "Hello world";

    let res1 = first_word(&s);
    let res2 = first_word(s2);
    println!("{}", res1);
    println!("{}", res2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(some_int: u32) {
    println!("{}", some_int);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(s: &mut String) {
    s.push_str(" world.");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
