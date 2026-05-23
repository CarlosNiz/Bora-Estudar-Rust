// fn main() {
//     // Loop infinito
//     //  
//     // loop {
//     //     println!("Hello, world!");
//     // }

//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }


// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is {element}");
//     }
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}");
//     }
// }

use std::io;

fn main() {
    println!("Sequência de fibonacci\n");

    println!("Digite a quantidade de números de fibonacci deseja ver:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro na leitura!");

    let input: u32 = input.trim().parse().expect("O valor deve ser um número natural");

    if input > 0 {
        fibonacci(input);
    } else {
        println!("O valor tem que ser maior que 0");
    }
}

fn fibonacci(number: u32) {
    let mut cont = 0;
    let mut term_one = 0;
    let mut term_two = 1;

    while cont < number {
        let result = term_one + term_two;

        println!("Fibonacci -> {}", term_one);

        term_one = term_two;
        term_two = result;

        cont += 1;
    }
}
