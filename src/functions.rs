pub fn main() {
    println!("main function!");
    another_function();
    print_labeled_measurement(5, 'h');
    teste_1_loop();
    teste_2_loop();
    teste_3_loop();
    operadores_teste()
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function() {
    println!("another function.");
}

fn operadores_teste() {
    println!("\nOperadores teste");
    let cond = true;
    let x;
    if cond {
        x = 1;
    } else {
        x = 2;
    }
    println!("x = {x}");
}

fn teste_1_loop() {
    println!("\nTeste 1 loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // saida do loop
        }
    };
    println!("The result is {result}");
}

fn teste_2_loop() {
    println!("\nTeste 2 loop");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn teste_3_loop() {
    println!("\nTeste 3 loop");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("----------");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("----------");
}
