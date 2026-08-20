mod models;
mod tela;

use models::cliente::Cliente;
use tela::menu as menu;
fn main() {
    let mut clientes = Vec::<Cliente>::new();
    menu::menu(&mut clientes);
}
