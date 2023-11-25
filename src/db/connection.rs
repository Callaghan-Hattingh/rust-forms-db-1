use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = {
        let conn = Connection::open("./rust-forms.db").expect("Failed to open the database");
        Mutex::new(conn)
    };
}

pub fn create_db_tables() -> rusqlite::Result<()> {
    let conn = DB_CONN.lock().unwrap();

    Ok(())
}
