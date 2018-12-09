fn psection(s: &str) {
    println!("----------- {} -----------", s);
}

fn main() {
    psection("random stuff");

    let s = String::from("hello"); // type: std::string::String (heap allocated?)
    let s2 = "hello"; // type: &str (stack allocated?)
    println!("{}{}", s, s2);

    let mut s = s;
    s.push_str(", world!");
    println!("{}", s);

    psection("try out move & clone");

    let s_dupe = s; // triggers a move (aka invalidates s)
    println!("{}", s_dupe);
    //println!("{}", s); // error
    let s_dupe_dupe = s_dupe.clone();
    println!("{}", s_dupe_dupe);
    println!("{}", s_dupe);

    psection("'takes_ownership' vs 'makes_copy' funcs");

    let s = String::from("hello");
    println!(
        "pre take ownership {} .. println! macro must do something special",
        s
    );
    takes_ownership(s);
    //println!("{}", s); // error: use of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    psection("return values can transfer ownership");

    let s1 = gives_ownership();
    println!("s1{}", s1);
    let s2 = String::from("hello");
    println!("s2{}", s2);
    let s3 = takes_and_gives_back(s2);
    //println!("s2{} s3{}", s2, s3); // s2 moved into s3. s2 invalid
    println!("s3{}", s3);
    println!("gg{}", gives_ownership_2());
    let s2 = gives_ownership_2();
    println!("s2{}", s2);

    psection("tuple & transferring ownership");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    let (len, s2) = calculate_length_2(s2);
    println!("The length of '{}' is {}.", s2, len);

    psection("string reference");
    let s1 = String::from("hi mom1");
    let len = calculate_length_by_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let s2: &String = &s1;
    //calculate_length(s1); // not allowed, compiler knows something still points at s1 after we attempt to move it here... wow
    println!("{}", s2);

    let mut s1 = String::from("hello");
    modify_string(&mut s1);
    println!("{}", s1);

    psection("mut code that generates errors");
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // let r2 = &s; // error: cannot borrow `s` as immutable because it is also borrowed as mutable
    //println!("{}, {}", r1, r2); // error
    //println!("{}, {}", s, r1); // error: cannot borrow `s` as immutable because it is also borrowed as mutable
    println!("{}", r1);
    let r1 = 3;
    //println!("{}, {}", s, r1); // shit error: cannot borrow `s` as immutable because it is also borrowed as mutable
    let mut s = String::from("ugh");
    {
        let r1 = &mut s;
        println!("inside scope, r1 borrows... {}", r1);
    }
    println!("{}, {}", s, r1);

    psection("multiple immutable references allowed");
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}{}{}{}", s, r1, r2, r3);
    //s.push_str(", world!"); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // r1 = &String::from("");
    // let r2 = 0;
    // let r3 = 0;
    // s.push_str(", world!");
    // println!("{}{}{}{}", s, r1, r2, r3);
}
fn modify_string(s: &mut String) {
    s.push_str(", world!");
}
fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    //(s, s.len()) // not valid, ownership of s transferred before s.len()
    (s, length)
}
fn calculate_length_2(s: String) -> (usize, String) {
    (s.len(), s)
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string
}
fn gives_ownership_2() -> String {
    String::from("world2")
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
