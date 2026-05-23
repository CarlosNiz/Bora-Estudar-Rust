use std::io;

fn main() {
    println!("\nBem vindo ao conversor de Celsius e Fahrenheit\n");
    let mut decision = true;

    while decision {
        let mut input_chose = String::new();
        println!("Qual você deseja converter?\n(1) Celsius -> Fahrenheit\n(2) Fahrenheit -> Celsius");

        io::stdin()
            .read_line(&mut input_chose)
            .expect("Erro ao ler valor!");

        match input_chose.trim() {
            "1" => {
                celsius_fahrenheit();
                decision = false;
            }

            "2" => {
                fahrenheit_celsius();
                decision = false;
            }

            _ => println!("\nValor digitado não esperado!\n")
        }
    }
}

fn celsius_fahrenheit() {
    println!("\nDigite o valor em Celsius para descobrir em Fahrenheit:");

    let celsius_fahrenheit: f64 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler valor");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\nValor digitado inválido\n");
                continue
            }
        };
    }; 

    println!("{celsius_fahrenheit} em Celsius é -> {}", ((celsius_fahrenheit * 1.8) + 32.0));
}

fn fahrenheit_celsius() {
    println!("\nDigite o valor em Fahrenheit para descobrir em Celcius:");

    let fahrenheit_number: f64 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler valor");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\nValor digitado inválido\n");
                continue
            }
        };
    };
  
    println!("{fahrenheit_number} em Celsius é -> {}", ((fahrenheit_number - 32.0) / 1.8));
}