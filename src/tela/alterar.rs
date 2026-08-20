use crate::{models::cliente::Cliente, tela::{ler, lista, menu, utils::{esperar, limpar_tela}}};

pub fn alterar_clientes(clientes: &mut Vec<Cliente>) {
    limpar_tela();
    if clientes.len() == 0 {
        println!("NENHUM CLIENTE CADASTRADO!");
        esperar(2);
        return;
    }

    let id = cap_id();
    lista::listar_clientes(clientes);
    if let Some(cliente) = buscar_cliente_por_id(clientes, id) {
        println!("ALTERANDO CLIENTE");
        println!("Digite o nome do cliente:");
        cliente.nome = ler::ler_dados();
        println!("Digite o CPF do cliente:");
        cliente.cpf = ler::ler_dados();
        println!("Digite o endereço do cliente:");
        cliente.endereco = ler::ler_dados();
        limpar_tela();
        println!("Cliente alterado com sucesso!");
        esperar(1);
        menu::menu(clientes);
    } else {
        limpar_tela();
        println!("CLIENTE NÃO ENCONTRADO!");
        esperar(2);
    }
}

fn buscar_cliente_por_id(clientes: &mut Vec<Cliente>, id: usize) -> Option<&mut Cliente> {
    clientes.iter_mut().find(|c| c.id == id)
}

fn cap_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente que deseja alterar");
    ler::ler_dados_int() as usize
}