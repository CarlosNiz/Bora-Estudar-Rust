use std::io;


fn gravar_texto_digitado(variavel: &mut String) {
    io::stdin()
        .read_line(variavel)
        .expect("Erro ao ler linha");
}


fn contar_palavras(texto: &str) -> u32 {
    let string_em_vetor = texto.as_bytes();
    let mut contador: u32 = 0;
    let mut dentro_de_palavra: bool = false;

    for &c in string_em_vetor.iter() {
        if c != b' ' && c != b'\t' && c != b'\n' {
            if !dentro_de_palavra {
                contador += 1;
                dentro_de_palavra = true;
            }
        } else {
            dentro_de_palavra = false;
        }
    }

    contador
}


fn primeira_palavra(texto: &str) -> String{
    if contar_palavras(texto) == 0 {
        return String::from("Não temos nenhum texto para extrair!")
    }
    let string_em_vetor = texto.as_bytes();
    let mut primeira_letra: usize = 0;
    let mut encontrei_primeira_letra: bool = false;

    for (indice, &item) in string_em_vetor.iter().enumerate() {
        if item == b' ' && encontrei_primeira_letra {
            return String::from_utf8_lossy(&string_em_vetor[primeira_letra..indice]).to_string().replace(" ", "");
        } else {
            if item != b' ' && encontrei_primeira_letra == false {
                encontrei_primeira_letra = true;
                primeira_letra = indice;
            }
            continue
        }
    }

    String::from_utf8_lossy(&string_em_vetor[..]).to_string().replace(" ", "")
}


fn main() {
    let mut texto_digitado = String::new();
    println!("Digite seu texto:");
    gravar_texto_digitado(&mut texto_digitado);
    println!();

    print!("Texto digitado: {texto_digitado}");
    print!("Total de palavras: {}\n", contar_palavras(&texto_digitado));
    print!("Primeira palavra: {}\n\n", primeira_palavra(&texto_digitado));
}