use diesel::{r2d2::ConnectionManager, SqliteConnection};
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn connect() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new("config/db.db");
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to connect to config/db.db")
}
