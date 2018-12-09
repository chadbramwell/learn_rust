fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("{}-{}", result, counter);

    let mut counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut counter = 0;
    while counter < a.len() {
        println!("the value is: {}", a[counter]);
        counter = counter + 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for (k, v) in a.iter().enumerate() {
        println!("the value at {} is {}", k, v);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    } //outputs 1, 2, 3
    println!("LIFTOFF!!!");
}
