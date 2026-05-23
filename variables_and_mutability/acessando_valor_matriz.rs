use std::io;

fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Digite o index que deseja acessar do array");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler o index");

    let index: usize = index.trim().parse().expect("O valor digitado não é um número");

    let element = a[index];

    println!("O valor do index {index} é: {element}");
}