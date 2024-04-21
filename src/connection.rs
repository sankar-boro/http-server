use std::env;
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::net::TcpStream;
use std::io::{Read, Write};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        self.stream.read(buffer).unwrap();
    }

    pub fn write(&mut self, buffer: &str) {
        self.stream.write(buffer.as_bytes()).unwrap();
    }

    pub fn close(&mut self) {
        self.stream.flush().unwrap();
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}


pub async fn pg_connection() -> Pool {

  let pg_db_name = env::var("PG_DB_NAME").unwrap();
  let pg_db_uname = env::var("PG_DB_UNAME").unwrap();
  let pg_db_password = env::var("PG_DB_PWD").unwrap();

  let mut cfg = Config::new();
  cfg.dbname = Some(pg_db_name);
  cfg.user = Some(pg_db_uname);
  cfg.password = Some(pg_db_password);
  cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
  let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
  println!("Connected to Postgres");
  pool
}