fn main() {
    //_read_and_print_input();
    _parse_tests();

    // options:
    //  * read in the entire file as a string, figure out how to get slices to string, pass to atoi
    //  * read file as I go, parse as I go, update value as I go
    // let contents = std::fs::read_to_string("input.txt").expect("failed opening input.txt");
    // let mut num: s32 = 0;
    // loop {
    //     let change = atoi::atoi::<s32>(contents);
    //     num += change;
    //     println!("{}", num);
    // }
    let now = std::time::Instant::now();
    let (sum, ms) = parse_whole_file();
    let total = now.elapsed().subsec_micros() as f32 / 1000.;
    println!("answer:{} | took {}ms, {}ms total", sum, ms, total);
}

fn _parse_tests() {
    let num = String::from("1");
    let nump: i32 = num.parse().expect("integer parsed");
    println!("{}", nump);

    let num = "2";
    let nump: i32 = num.parse().expect("integer parsed");
    println!("{}", nump);

    let nump: i32 = "3".parse().expect("integer parsed");
    println!("{}", nump);

    // panic
    //let nump: i32 = "4\n".parse().expect("integer parsed");
    //println!("{}", nump);

    // panic
    //let nump: i32 = "\n5".parse().expect("integer parsed");
    //println!("{}", nump);

    let nump: i32 = "\n6".trim().parse().expect("integer parsed");
    println!("{}", nump);

    let nump: i32 = "\n7\n".trim().parse().expect("integer parsed");
    println!("{}", nump);

    let nump: i32 = "\n\n8\n\r\n".trim().parse().expect("integer parsed");
    println!("{}", nump);

    // panic
    //let nump: i32 = "\n9\n10".trim().parse().expect("integer parsed");
    //println!("{}", nump);
}

fn _read_and_print_input() {
    let contents = std::fs::read_to_string("input.txt").expect("failed opening input.txt");
    println!("{}", contents);
}

// Attempt #1
// * read whole file into a string
// * then parse each integer (skipping 2 characters after each for \r\n)
// * then sum each integer
// Confusing/Interesting stuff:
// * timing the whole function call vs internal pieces has large discrepencies when using
//   println! (missing ~0.13ms!). Comment out both println!s to see time disappear.
//   cargo run --release makes no different in this time
// * I suspect the cost of allocating and deallocating strings for println! is what's taking
//   my time. But I haven't worked out yet how to format messages on the stack
// * cargo run --release drops algo time from (0.4 -> 0.02) but has no change on reading file
// For Attempt #2
// * Revisit when I learn how to pull integers out of a string of a bunch of numbers
// * Revisit when I learn how to read a file in chunks
fn parse_whole_file() -> (i32, f32) {
    let now = std::time::Instant::now();
    let contents = std::fs::read_to_string("input.txt").expect("failed to open input.txt");
    let file_ms = now.elapsed().subsec_micros() as f32 / 1000.;
    println!("reading file into memory took {}ms", file_ms);
    let now = std::time::Instant::now();
    let bytes = contents.as_bytes();
    let mut sum: i32 = 0;
    let mut last_i = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\r' {
            let slice = &contents[last_i..i];
            //println!("{}", slice);
            let value: i32 = slice.parse().expect("parsed integer");
            //println!("{} -> {}", slice, value);
            sum = sum + value;
            last_i = i + 2; //+2 for \r\n <- bad form. a more correct version would be to seek until we find non-whitespace
        }
    }
    let slice = &contents[last_i..];
    //println!("{}", slice);
    let value: i32 = slice.parse().expect("parsed integer");
    //println!("{} -> {}", slice, value);
    sum = sum + value;

    let algo_ms = now.elapsed().subsec_micros() as f32 / 1000.;
    let now = std::time::Instant::now();
    println!("algorithm took {}ms", algo_ms);
    //std::io::Write::write!(&mut std::io::stdout(), "algorithm took {}ms", algo_ms);
    let last_ms = now.elapsed().subsec_micros() as f32 / 1000.;
    (sum, (file_ms + algo_ms + last_ms))
}
