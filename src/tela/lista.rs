use crate::models::cliente::Cliente;
use crate::tela::utils::*;

pub fn listar_clientes(clientes: &Vec<Cliente>){
    limpar_tela();
    if clientes.len() == 0{
        println!("NENHUM CLIENTE CADASTRADO!");
        esperar(2);
        return;
    } else {
        println!("LISTA DE CLIENTES:");
        for cliente in clientes{
            println!("ID: {}", cliente.id);
            println!("NOME: {}", cliente.nome);
            println!("CPF: {}", cliente.cpf);
            println!("ENDEREÇO: {}", cliente.endereco);
            println!("-----------------------------");
        }
    }
    esperar(1);
    return;
}