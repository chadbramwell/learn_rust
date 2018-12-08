fn main() {
    println!(
"What would you like to do?
1) Convert temperatures between Fahrenheit and Celsius
2) Generate the nth Fibonacci number.
3) Print the lyrics to the Christmas carol \"The Twelve Days of Christmas,\" taking advantage of the repetition in the song.");

    let choice: u32 = {
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("can this fail?");
        choice.trim().parse().expect("invalid choice!")
    };
    match choice {
        1 => convert_temperature(),
        2 => generate_fibonacci(),
        3 => christmas_carol(),
        _ => println!("todo.."),
    }
}

fn convert_temperature() {
    println!("Please provide degrees Fahrenheit");
    let mut f = String::new();
    std::io::stdin().read_line(&mut f).expect("can this fail?");
    let f: f32 = f.trim().parse().expect("invalid number!");
    println!("{}° is {}° Centigrade", f, (f - 32.) * 5. / 9.);
}

fn generate_fibonacci() {
    println!("Please provide an integer");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("can this fail?");
    let n: u32 = n.trim().parse().expect("invalid number!");

    let nth = {
        let mut f0 = 0;
        let mut f1 = 1;
        for i in 0..n {
            println!("[{}] {}", i, f0);
            let temp = f0;
            f0 = f1;
            f1 = temp + f1;
        }
        f0
    };
    println!("{}th is {}", n, nth);
}

fn christmas_carol() {
    let stanzas = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLDEN RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[day]
        );
        for revday in (0..day + 1).rev() {
            if revday == 0 && day != 0 {
                print!("And ");
            }
            println!("{}", stanzas[revday]);
        }
    }
}
