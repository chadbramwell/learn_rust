fn main() {
    let r = no_dangle();
    println!("{}", r);
}
fn no_dangle() -> String {
    // returns ownership
    String::from("yo")
}
// more about lifetimes in Ch10
// fn dangle() -> &String { // returns reference (but not ownership, because ownership is lost... error: missing lifetime specifier)
//     &String::from("yo")
// }
// from docs (https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html):
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!
