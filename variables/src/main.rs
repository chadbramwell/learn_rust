//const fn get_points() -> u32 {
//    return 5;
//}
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000; //get_points();
    println!("Max POINTS: {}", MAX_POINTS);

    let x = 7;
    println!("The value of x is : {}", x);
    let x = x * x;
    p(x);
    p(x / x);
    p(MAX_POINTS);

    let x = format!("hello bot #{}", x);
    println!("{}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{} spaces", spaces);

    let tup = (500, 6.4, '⏚');
    let (x, _, _) = tup;
    println!("{}{}{}", x, tup.1, tup.2);
}
fn p(x: u32) {
    println!("p of x is: {}", x);
}
