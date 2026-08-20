# ClientesCrud

Pequeno aplicativo CLI/TUI em Rust para gerenciar um cadastro de clientes (CRUD).

## Recursos
- Listar clientes
- Ler detalhes de um cliente
- Cadastrar novo cliente
- Alterar cliente existente
- Excluir cliente

## Requisitos
- Rust toolchain (stable) instalado — https://rustup.rs

## Build e execução
1. Baixe as dependências e construa em modo debug:

   cargo build

2. Execute o aplicativo:

   cargo run

3. Para build de produção:

   cargo build --release

## Estrutura do projeto
- [src/main.rs](src/main.rs) — ponto de entrada
- [src/tela/menu.rs](src/tela/menu.rs) — menu principal e navegação
- [src/tela/lista.rs](src/tela/lista.rs) — tela de listagem
- [src/tela/ler.rs](src/tela/ler.rs) — visualizar cliente
- [src/tela/cadastro.rs](src/tela/cadastro.rs) — cadastro de cliente
- [src/tela/alterar.rs](src/tela/alterar.rs) — alteração de cliente
- [src/tela/excluir.rs](src/tela/excluir.rs) — exclusão de cliente
- [src/tela/utils.rs](src/tela/utils.rs) — utilitários da UI
- [src/enums](src/enums) — enums do projeto
- [src/models/cliente.rs](src/models/cliente.rs) — definição do modelo `Cliente`

## Uso
Ao executar `cargo run` o aplicativo apresenta um menu interativo no terminal. Navegue pelas opções para listar, visualizar, cadastrar, alterar ou excluir clientes.

## Contribuição
PRs são bem-vindos. Abra uma issue descrevendo o problema/feature antes de implementar mudanças maiores.

## Licença
Projeto disponibilizado sem licença explícita. Adicione uma `LICENSE` se desejar uma licença permissiva (ex: MIT).
