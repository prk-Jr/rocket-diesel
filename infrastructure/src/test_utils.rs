use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
        .build(manager)
        .expect("Error creating test connection pool");
    let mut conn = pool.get().expect("Error getting test connection");
    diesel::sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, first_name TEXT NOT NULL, last_name TEXT NOT NULL)")
        .execute(&mut conn)
        .unwrap();
    diesel::sql_query("CREATE TABLE posts (id INTEGER PRIMARY KEY AUTOINCREMENT, created_by INTEGER, title TEXT NOT NULL, body TEXT NOT NULL, FOREIGN KEY(created_by) REFERENCES users(id))")
        .execute(&mut conn)
        .unwrap();
    diesel::sql_query("CREATE TABLE posts_tags (post_id INTEGER NOT NULL, tag TEXT NOT NULL, PRIMARY KEY (post_id, tag), FOREIGN KEY(post_id) REFERENCES posts(id))")
        .execute(&mut conn)
        .unwrap();
    pool
}