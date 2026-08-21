# ClientesCrud

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MySQL](https://img.shields.io/badge/MySQL-8.x-4479A1?logo=mysql&logoColor=white)](https://www.mysql.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

Aplicação CLI em Rust para cadastrar e administrar clientes em um banco MySQL.

O programa apresenta um menu interativo no terminal e implementa as operações de criar, listar, atualizar e excluir clientes. As consultas CRUD são geradas a partir do modelo por meio do crate local `model_macro`.

## Screenshots

<div align="center">
  <img src="TelaPrincipal.png" alt="Menu principal do ClientesCrud" width="900" />
</div>

### Exemplos do fluxo do sistema

<div align="center">
  <img src="ListarClientes.png" alt="Listagem de clientes" width="430" />
  <img src="Cadastrar.png" alt="Cadastro e atualização de clientes" width="430" />
</div>

## Funcionalidades
- Criar clientes
- Listar clientes
- Atualizar clientes existentes
- Excluir clientes
- Conectar ao MySQL usando `mysql` e carregar variáveis com `dotenv`
- Gerar SQL CRUD a partir da definição do modelo

## Requisitos
- Rust stable com edição 2024
- MySQL ou MariaDB instalado e em execução
- Cliente de linha de comando `mysql` disponível para restaurar o banco
- Acesso a um banco local ou a uma instância configurada em `DATABASE_URL`

## Configuração do banco
O projeto usa `DATABASE_URL` para conectar ao MySQL. Se a variável não estiver definida, a aplicação usa:

```bash
mysql://root:root@localhost:3306/clientes_rust_db
```

Você pode criar um arquivo `.env` na raiz do projeto com:

```env
DATABASE_URL=mysql://root:root@localhost:3306/clientes_rust_db
```

Para preparar o banco com o script incluído, ajuste as credenciais no próprio `db/migrate.sh` se necessário e execute, a partir da raiz do projeto:

```bash
bash db/migrate.sh
```

O script usa, por padrão, o usuário `root`, a senha `root`, o host `localhost` e o banco `clientes_rust_db`. Também é possível importar o SQL diretamente:

```bash
mysql -u root -proot < db/restore.sql
```

O arquivo SQL cria o banco `clientes_rust_db`, a tabela `clientes` e três registros iniciais. A tabela contém:

| Coluna | Tipo | Observação |
| --- | --- | --- |
| `id` | `INT` | Chave primária com incremento automático |
| `nome` | `VARCHAR(255)` | Obrigatório |
| `telefone` | `VARCHAR(20)` | Obrigatório |

## Build e execução
Compile o projeto e execute o menu com:

```bash
cargo build
cargo run
```

Para gerar uma versão otimizada:

```bash
cargo build --release
```

## Menu da aplicação
Ao executar o programa, o terminal apresenta as opções:

1. Criar Cliente
2. Listar Clientes
3. Atualizar Cliente
4. Excluir Cliente
5. Sair

## Estrutura do projeto
- [src/main.rs](src/main.rs) — ponto de entrada e menu principal
- [src/config/cnn.rs](src/config/cnn.rs) — configuração e conexão com o banco
- [src/models/cliente.rs](src/models/cliente.rs) — modelo `Cliente` e metadados da tabela
- [src/repositorios/generico_repositorio.rs](src/repositorios/generico_repositorio.rs) — operações CRUD genéricas no MySQL
- [src/tela/tela.rs](src/tela/tela.rs) — fluxo de interação com o usuário
- [model_macro/src/lib.rs](model_macro/src/lib.rs) — macro que gera o modelo e as consultas SQL
- [db/restore.sql](db/restore.sql) — criação do banco, da tabela e dos dados iniciais
- [db/migrate.sh](db/migrate.sh) — script para restaurar o banco

## Observações
- O projeto é um CLI e não possui interface gráfica.
- A aplicação não cria automaticamente o banco nem a tabela ao iniciar; prepare o banco antes de executar `cargo run`.
- A conexão padrão está definida em [src/config/cnn.rs](src/config/cnn.rs) e pode ser substituída por `DATABASE_URL` no arquivo `.env`.

## Licença
Este projeto está licenciado sob a [MIT License](LICENSE).

