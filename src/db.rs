use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use lazy_static::lazy_static;
use crate::errors::ApiError;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

embed_migrations!();

lazy_static!{
    static ref POOL: MysqlPool = {
        let conn = std::env::var("DB_CONNECTION").expect("DB_CONNECTION must be set");
        let host = std::env::var("DB_HOST").expect("DB_HOST must be set");
        let port = std::env::var("DB_PORT").expect("DB_PORT must be set");
        let dbname = std::env::var("DB_DATABASE").expect("DB_DATABASE must be set");
        let username = std::env::var("DB_USERNAME").expect("DB_USERNAME must be set");
        let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        
        let database_url = format!("{}://{}:{}@{}:{}/{}", conn, username, password, host, port, dbname);

        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create db pool.")
    };
}

pub fn init() {
    info!("Initializing Database...");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection.");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}