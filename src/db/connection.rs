use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;

use super::{load::LoadDB, truck::TruckDB, Table, Column, TableDB};

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

    let table = Table {
        tablename: "test".to_string(),
        columns: vec![
            Column {
                name: "field1".to_string(),
                datatype: "BigInt".to_string(),
            },
            Column {
                name: "field2".to_string(),
                datatype: "String".to_string(),
            },
        ],
    };
    let table_db = TableDB::new(&conn);
    let _ = table_db.dynamic_create_table(table);

    Ok(())
}
