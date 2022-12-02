use std::env;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager, PoolError};

use super::Errors;

/// Create PgPool type which is basically a `Pool<ConnectionManager<PgConnection>>`
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Create PgPooledConnection type which is basically a `PooledConnection<ConnectionManager<PgConnection>>`
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Database pool enum which will allows the actix web server
/// to run with or without proper database connection
#[derive(Clone)]
pub enum DBPool {
    Postgres(PgPool),
    Others
}

/// DBPool implementations
impl DBPool {
    /// Set new DBPool instance
    ///
    /// Example
    /// ```
    /// use library::{databases, DBPool};
    ///
    /// fn main() {
    ///     // Set database pool instance
    ///     let mut pool = DBPool::Others;
    ///     let result = databases::stage();
    /// }
    /// ```
    pub fn new(pool: PgPool) -> Self {
        Self::Postgres(pool)
    }

    /// Get database from r2d2 pool
    ///
    /// Example
    /// ```
    /// use library::{databases, DBPool};
    ///
    /// fn main() {
    ///     // Set database pool instance
    ///     let result = databases::stage();
    ///
    ///     if result.is_ok() {
    ///         // Set pool by shadowing the initial pool declaration
    ///         let pool = DBPool::new(result.unwrap().clone());
    ///
    ///         // Get database connection from pool
    ///         let conn = pool.get();
    ///     }
    /// }
    /// ```
    pub fn get(&self) -> Result<PgPooledConnection, Errors> {
        return match self {
            DBPool::Postgres(_pool) => {
                let pool = _pool.get();
                if pool.is_err() {
                    return Err(Errors::new("Unable to initialize your database pool"));
                }

                let conn:PgPooledConnection = pool.unwrap();

                Ok(conn)
            },
            DBPool::Others => Err(Errors::new("Unable to initialize your database pool"))
        }
    }
}

/// Returns a connection from the PgPool directly
pub fn pool_conn(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    pool.get()
}

/// Connects to Postgres and call init pool
pub fn stage() -> Result<PgPool, Errors> {
    // Set database url
    let result = env::var( "DATABASE_URL");
    if result.is_err() {
        return Err(Errors::new("Failed to parse DATABASE_URL. Please make sure you had a valid env value"));
    }

    // Set url
    let url = result.unwrap();

    // Create a default R2D2 Postgres DB Pool
    let manager = ConnectionManager::<PgConnection>::new(url);
    let builder = Pool::builder().build(manager);
    if builder.is_err() {
        return Err(Errors::new("Unable to initialize your database pool"));
    }

    // Return builder result
    Ok(builder.unwrap())
}