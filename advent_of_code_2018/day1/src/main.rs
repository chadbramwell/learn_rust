fn main() {
    //_read_and_print_input();

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
    let sum = parse_whole_file();
    println!("{}", sum);
}

fn _read_and_print_input() {
    let contents = std::fs::read_to_string("input.txt").expect("failed opening input.txt");
    println!("{}", contents);
}

fn parse_whole_file() -> i32 {
    let contents = std::fs::read_to_string("input.txt").expect("failed to open input.txt");
    let bytes = contents.as_bytes();
    let mut sum: i32 = 0;
    let mut last_i = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\r' {
            let slice = &contents[last_i..i];
            //println!("{}", slice);
            let value: i32 = slice.parse().expect("parsed integer");
            println!("{} -> {}", slice, value);
            sum = sum + value;
            last_i = i + 2; //+2 for \r\n
        }
    }
    let slice = &contents[last_i..];
    //println!("{}", slice);
    let value: i32 = slice.parse().expect("parsed integer");
    println!("{} -> {}", slice, value);
    sum = sum + value;

    sum
}
