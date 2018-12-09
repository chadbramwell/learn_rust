extern crate atoi;

fn main() {
    //_read_and_print_input();
    read_and_print_input_line_by_line();
}

fn _read_and_print_input() {
    let contents = std::fs::read_to_string("input.txt").expect("failed opening input.txt");
    println!("{}", contents);
}

fn read_and_print_input_line_by_line() {
    // options:
    //  1) read in the entire file as a string, figure out how to split by newline, process each string
    //  2) read in the entire file as a string, figure out how to get slices to string, pass to atoi
    //  3) read file as I go, parse as I go, update value as I go
    // let contents = std::fs::read_to_string("input.txt").expect("failed opening input.txt");
    // let mut num: s32 = 0;
    // loop {
    //     let change = atoi::atoi::<s32>(contents);
    //     num += change;
    //     println!("{}", num);
    // }
}
