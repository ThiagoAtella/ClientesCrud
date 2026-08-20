use crate::models::cliente::Cliente;
use crate::tela::alterar;
use crate::tela::cadastro;
use crate::tela::excluir;
use crate::tela::lista;
use crate::tela::ler::*;
use crate::tela::utils::*;

pub fn menu(clientes: &mut Vec<Cliente>){
    loop{
        limpar_tela();
        println!("\
        =============MENU=============\n\
        ESCOLHA UMA OPÇÃO:\n\
        1 - CADASTRAR CLIENTE\n\
        2 - ALTERAR CLIENTE\n\
        3 - EXCLUIR CLIENTE\n\
        4 - LISTAR CLIENTES\n\
        0 - SAIR\n\
        ");
        let opcao = ler_dados_int();
        match opcao {
            1 => {
                println!("CADASTRAR CLIENTE");
                esperar(1);
                cadastro::cadastrar_cliente(clientes);
            },
            2 => {
                println!("ALTERAR CLIENTE");
                alterar::alterar_clientes(clientes);
            },
            3 => {
                println!("EXCLUIR CLIENTE");
                excluir::excluir_clientes(clientes);
            },
            4 => {
                println!("LISTAR CLIENTES");
                esperar(1);
                lista::listar_clientes(clientes);
            },
            0 => {
                println!("SAINDO...");
                break;
            },
            _ => {
                println!("OPÇÃO INVÁLIDA!");
                esperar(2);
            }
        }
        esperar(2);
    }
}