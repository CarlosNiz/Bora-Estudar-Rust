// fn main() {
//     let mut s = String::from("Hello, world");
    
//     let size = first_word(&s); // nada garante que após apagarmos ou mudarmos o conteúdo essa variável continue válida

//     println!("{size}");

//     s.clear();

//     println!("{s}");
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         } 
//     }

//     s.len()
// }


// fn main() {
//     let s = String::from("hello world!");

//     // .. -> sintaxe de intervalo
//     let hello = &s[..5];
//     let world  = &s[6..11];
//     // agora as variáveis representam um intervalo da variável s
    

// }

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&mut s);
    println!("the first word is: {word}");   // último uso de `word` aqui

    s.clear();                               // agora pode: `word` já não é mais usado
    println!("s agora está vazio: {s:?}");

    //
    //  fatias também servem para matriz
    //

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &mut str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}