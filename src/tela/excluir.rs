use crate::models::cliente::Cliente;
use crate::tela::ler::*;
use crate::tela::lista;
use crate::tela::utils::*;

pub fn excluir_clientes(clientes: &mut Vec<Cliente>) {
     if clientes.is_empty() {
         println!("Não há clientes cadastrados.");
         esperar(2);
         return;
     }
 
     let id = cap_id();
     lista::listar_clientes(clientes);
     if let Some(pos) = clientes.iter().position(|c| c.id == id) {      
         clientes.remove(pos);
         println!("Cliente excluído com sucesso!");
     } else {
         println!("Cliente não encontrado.");
     }
     esperar(2);
 }
 fn cap_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente que deseja excluir");
    ler_dados_int() as usize
}