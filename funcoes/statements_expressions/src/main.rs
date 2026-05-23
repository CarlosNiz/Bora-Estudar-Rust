fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {y}");

    let x = plus_one(1);
    println!("{x}");
}

fn plus_one(number: i32) -> i32 {
    number + 1
}