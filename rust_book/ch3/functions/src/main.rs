fn main() {
    println!("Hello, world!");

    another_function();

    let y = {
        let x = 3;
        x + 1 // oh sure, this is okay but return x + 1; is not >_>
    };
    println!("y={}", y);

    let a = [1, 3, 5];
    println!("{}", a[1]);

    let f1 = five();
    let f2 = five_again();
    println!("5{}5{}", five(), five_again());
    println!("5{}5{}", f1, f2);

    // let search_result: i32 = {
    //     for (k:i32, v) in a.iter().enumerate() {
    //         if v == 3 {
    //             k
    //         }
    //     }
    //     -1
    // };
    // println!("{}", search_result);

    let x = 127;
    match x {
        0 => println!("0!"),
        1 => println!("1"),
        2...20 => println!("sure"),
        _ => (),
    };
}
fn another_function() {
    println!("Another function.");
}
// fn five() -> i32 { // this works same as below without any other code change.
//     5
// }
fn five() -> std::string::String {
    "5".to_string()
}
fn five_again() -> i32 {
    return 5;
}
