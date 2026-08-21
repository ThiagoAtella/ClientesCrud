use mysql::{Pool, Opts, PooledConn};
use dotenv::dotenv;
use std::env;

pub fn  obter_conexao() -> Result<PooledConn, mysql::Error>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root:root@localhost:3306/clientes_rust_db".to_string());
    let opts = Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}