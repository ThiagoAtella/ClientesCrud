# ClientesCrud

Versão atual: 0.1.0

Aplicativo em Rust para gerenciar um cadastro de clientes por meio de um menu interativo no terminal, com persistência em MySQL.

## Funcionalidades
- Criar cliente
- Listar clientes
- Atualizar cliente existente
- Excluir cliente
- Conexão com banco via MySQL e dotenv

## Requisitos
- Rust (stable)
- MySQL ou MariaDB instalado e em execução
- Banco de dados acessível localmente

## Configuração do banco
O projeto usa `DATABASE_URL` para conectar ao MySQL. Se a variável não estiver definida, o código usa a seguinte URL padrão:

```bash
mysql://root:root@localhost:3306/clientes_rust_db
```

Você pode criar um arquivo `.env` na raiz do projeto com:

```env
DATABASE_URL=mysql://root:root@localhost:3306/clientes_rust_db
```

Para preparar o banco, execute o script em `db/`:

```bash
cd db
bash migrate.sh
```

Ou importe o SQL manualmente:

```bash
mysql -u root -proot < db/restore.sql
```

## Build e execução
1. Instale as dependências do projeto:

```bash
cargo build
```

2. Execute a aplicação:

```bash
cargo run
```

3. Para gerar a versão de produção:

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
- [src/models/cliente.rs](src/models/cliente.rs) — modelo `Cliente`
- [src/repositorios/cliente_repositorio.rs](src/repositorios/cliente_repositorio.rs) — operações CRUD no MySQL
- [src/tela/tela.rs](src/tela/tela.rs) — fluxo de interação com o usuário
- [db/restore.sql](db/restore.sql) — criação do banco e dados iniciais
- [db/migrate.sh](db/migrate.sh) — script para restaurar o banco

## Observações
- O projeto foi desenvolvido como um CLI simples e não possui interface gráfica.
- A estrutura do banco atual é baseada na tabela `clientes` com colunas `id`, `nome` e `telefone`.

