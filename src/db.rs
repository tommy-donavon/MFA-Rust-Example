use std::{env, error::Error,time::Duration};
use diesel::{SqliteConnection, r2d2::{ConnectionManager,CustomizeConnection, Error as PoolError}, r2d2};
use diesel::connection::SimpleConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
struct ConnectionOptions {
    enable_wal: bool,
    enable_foreign_keys: bool,
    busy_timeout: Option<Duration>,
}

impl CustomizeConnection<SqliteConnection, PoolError> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), PoolError> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }
            Ok(())
        })().map_err(PoolError::QueryError)
    }
}

pub fn establish_connection(enable_wal:bool, enable_foreign_keys:bool, busy_timeout:Option<Duration>) -> Result<Pool,Box<dyn Error>> {
    let conn_ops = Box::new(ConnectionOptions {
        enable_wal,
        enable_foreign_keys,
        busy_timeout
    });
    let database_url:String = env::var("DATABASE_URL")?;
    let database_pool = Pool::builder()
        .max_size(16)
        .connection_customizer(conn_ops)
        .build(ConnectionManager::<SqliteConnection>::new(database_url))?;
    Ok(database_pool)
}