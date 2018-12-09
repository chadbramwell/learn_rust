fn main() {
    let s = String::from("hello");
    assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes()); // run-time assert :(

    let s = String::from("hello, world!");
    println!("{}", first_word_length(&s));

    let hello = &s[0..5];
    let world = &s[7..12];
    let end = &s[7..];
    let hello2 = &s[0..=4];
    println!("{}, {}, {}, {}", hello, world, end, hello2);

    let hello = &s[..5];
    let hw = &s[0..s.len()];
    let hw2 = &s[..];
    println!("{} | {} | {}", hello, hw, hw2);

    let s2 = String::from("⛉");
    //println!("{}", &s2[..1]); // runtime crash: string slices must land on UTF8 character boundaries
    println!("{}", &s2[..("⛉".as_bytes().len())]);

    println!("{} | {}", first_word(&s), first_word(&s2));
}
fn first_word(s: &str) -> &str {
    // String can be passed into this as well as string literals!
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes(); // as_bytes returns a slice
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
