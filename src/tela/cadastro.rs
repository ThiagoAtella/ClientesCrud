use crate::tela::ler::*;
use crate::tela::menu::menu;
use crate::tela::utils::*;
use crate::models::cliente::Cliente;

pub fn cadastrar_cliente(clientes: &mut Vec<Cliente>){
    limpar_tela();
    println!("CADASTRAR CLIENTE");
    let mut cliente: Cliente = Cliente::default();
    cliente.id = clientes.len() + 1;
    println!("Digite o nome do cliente:");
    cliente.nome = ler_dados();
    println!("Digite o CPF do cliente:");
    cliente.cpf = ler_dados();
    println!("Digite o endereço do cliente:");
    cliente.endereco = ler_dados();
    clientes.push(cliente);
    limpar_tela();
    println!("Cliente cadastrado com sucesso!");
    esperar(1);
    menu(clientes);
}