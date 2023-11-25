use rusqlite::Connection;
use std::sync::MutexGuard;

#[derive(Debug)] 
pub struct Load {
    pub id: u32,
    pub height: f64,
    pub length: f64,
    pub quantity: f64,
    pub width: f64,
}

pub struct LoadDB<'a> {
    conn: &'a MutexGuard<'a, Connection>,
}

impl LoadDB<'_> {
    pub fn new<'a>(conn: &'a MutexGuard<'a, Connection>) -> LoadDB<'a> {
        LoadDB{ conn }
    }

    pub fn create_load_db_table(&self) -> rusqlite::Result<()> {
        self.conn.execute(
            "create table if not exists load (
                id integer primary key,
                height float not null,
                length float not null,
                quantity float not null,
                width float not null
                )",
                (),
        )?;
        Ok(())
    }
}