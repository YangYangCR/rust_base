fn main() {
    let num = 4;
    if num < 5 {
        println!("The number is less than five");
    } else {
        println!("The number is greater than five");
    }

    if num != 0 {
        println!("number was not zero");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }
    println!("The result is {}", counter);


    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{}", element);
    }

    println!("=========================");
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
