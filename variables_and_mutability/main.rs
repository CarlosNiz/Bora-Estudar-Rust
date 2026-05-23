const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    println!("\nTrês horas em segundos: {THREE_HOURS_IN_SECONDS}");

    println!("\n\n\n");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("O valor de Y nesse escopo é: {y}");
    }

    println!("O valor de y é: {y}");
}