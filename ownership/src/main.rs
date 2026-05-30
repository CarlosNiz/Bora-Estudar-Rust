// fn main() {
//     {
//         let s = "hello";
//         println!("{s}");
//     } // a variável s só é válida dentro do escopo

//     let mut s = String::from("hello");
//     s.push_str(", world!");
//     println!("{s}");
// }


// fn main() {
//     let s1 = String::from("hello");
//     // let s2 = s1; isso vai falhar, pois ao fechar o escopo o s1 tenta ser excluído, porém ele já não existe mais quando passamos s1 para s2 
//     let s2 = s1.clone(); // esse é o correto caso precise

//     println!("s1 = {s1}, s2 = {s2}");
// }


// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); 

//     (s, length)
// }


// fn main() {
//     let s = String::from("hello");

//     let size: usize = calcular_tamanho_string(&s);

//     println!("O tamanho de '{s}' é {size}");
// }

// fn calcular_tamanho_string(s: &String) -> usize {
//     s.len()
// }


fn main() {
    let mut texto = String::from("Olá");

    change_string(&mut texto); 

    println!("{texto}");
}

fn change_string(referencia_mutavel_string: &mut String) {
    referencia_mutavel_string.push_str(", mundo!");
}