use crate::repositorios::cliente_repositorio;
use std::io;


pub fn criar_cliente(){
    let mut nome = String::new();
    let mut telefone = String::new();
    println!("Digite o nome do cliente:");
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");
    println!("Digite o telefone do cliente:");
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");
    cliente_repositorio::criar(nome.trim(), telefone.trim()).expect("Falha ao criar o cliente");
    println!("Cliente criado com sucesso!");
}

pub fn mostrar_clientes() -> Result<(), Box<dyn std::error::Error>> {
    let clientes = cliente_repositorio::listar()?;
    for cliente in clientes {
        println!("----------------------------------"); // Risco na tela
        println!("ID: {}", cliente.id);
        println!("Nome: {}", cliente.nome);
        println!("Telefone: {}", cliente.telefone);
    }
    Ok(())
}

pub fn atualizar_cliente() -> Result<(), Box<dyn std::error::Error>> {
    let mut id = String::new();
    let mut nome = String::new();
    let mut telefone = String::new();

    println!("Digite o ID do cliente que deseja atualizar:");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id: u32 = id.trim().parse().expect("ID inválido");

    println!("Digite o novo nome do cliente:");
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Digite o novo telefone do cliente:");
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");

    cliente_repositorio::atualizar(id, nome.trim(), telefone.trim())?;
    println!("Cliente atualizado com sucesso!");
    Ok(())
}
pub fn excluir_cliente() -> Result<(), Box<dyn std::error::Error>> {
    let mut id = String::new();
    println!("Digite o ID do cliente que deseja excluir:");
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");
    let id: u32 = id.trim().parse().expect("ID inválido");

    cliente_repositorio::deletar(id)?;
    println!("Cliente excluído com sucesso!");
    Ok(())
}
