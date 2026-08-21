use mysql::*;
use mysql::prelude::Queryable;
use crate::models::cliente::Cliente;
use crate::config::cnn::obter_conexao;

pub fn criar(nome: &str, telefone: &str) -> Result<(), mysql::Error>{
    let mut conn = obter_conexao()?;
    conn.exec_drop(
        r"INSERT INTO clientes(nome, telefone) VALUES (:nome, :telefone)",
        params!{
            "nome" => nome,
            "telefone" => telefone,
        })?;
        Ok(())
}

pub fn listar() -> Result<Vec<Cliente>, mysql::Error>{
    let mut conn = obter_conexao()?;
    let clientes: Vec<Cliente> = conn.query_map(
        "SELECT id, nome, telefone FROM clientes",
        |(id, nome, telefone)|{
            Cliente {
                id: id,
                nome: nome,
                telefone: telefone,
            }
        }
    )?;
    Ok(clientes)
}

pub fn atualizar(id: u32, nome: &str, telefone: &str) -> Result<(), mysql::Error>{
    let mut conn = obter_conexao()?;
    conn.exec_drop(
        r"UPDATE clientes SET nome = :nome, telefone = :telefone WHERE id = :id",
        params!{
            "id" => id,
            "nome" => nome,
            "telefone" => telefone,
        }
    )?;
    Ok(())
}
pub fn deletar(id: u32) -> Result<(), mysql::Error>{
    let mut conn = obter_conexao()?;
    conn.exec_drop(
        r"DELETE FROM clientes WHERE id = :id",
        params!{
            "id" => id,
        }
    )?;
    Ok(())
}