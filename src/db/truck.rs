use rusqlite::Connection;
use std::sync::MutexGuard;

#[derive(Debug)] 
pub struct Truck {
    pub id: u32,
    pub mass: f64,
    pub length: f64,
    pub quantity: f64,
    pub capicity: f64,
    pub wheels: i32,
}

pub struct TruckDB<'a> {
    conn: &'a MutexGuard<'a, Connection>,
}

impl TruckDB<'_> {
    pub fn new<'a>(conn: &'a MutexGuard<Connection>) -> TruckDB<'a> {
        TruckDB{ conn }
    }

    pub fn create_truck_db_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "create table if not exists truck (
                id integer primary key,
                mass float not null,
                length float not null,
                capicity float not null,
                width float not null,
                wheels integer not null
                )",
                (),
        )?;
        Ok(())
    }
}