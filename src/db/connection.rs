use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;

use super::{load::LoadDB, truck::TruckDB};

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = {
        let conn = Connection::open("./rust-forms.db").expect("Failed to open the database");
        Mutex::new(conn)
    };
}

pub fn create_db_tables() -> rusqlite::Result<()> {
    let conn = DB_CONN.lock().unwrap();
    let load_db = LoadDB::new(&conn);
    let _ = load_db.create_load_db_table();
    let truck_db = TruckDB::new(&conn);
    let _ = truck_db.create_truck_db_table();
    
    Ok(())
}
