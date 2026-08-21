CREATE DATABASE IF NOT EXISTS clientes_rust_db;
use clientes_rust_db;

create table if not exists clientes(
    id int auto_increment primary key,
    nome varchar(255) not null,
    telefone varchar(20) not null
);

insert into clientes (nome, telefone) values
('João Silva', '123456789'),
('Maria Souza', '987654321'),
('Carlos Oliveira', '456789123');