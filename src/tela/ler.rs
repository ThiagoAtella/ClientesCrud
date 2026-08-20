
pub fn ler_dados() -> String {
    let mut dados = String::new();
    std::io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    return dados.trim().to_string();
}

pub fn ler_dados_int() -> i32 {
    let mut dados = String::new();
    std::io::stdin().read_line(&mut dados).expect("Falha ao ler dados");
    return dados.trim().parse().expect("Falha ao converter para inteiro");
}