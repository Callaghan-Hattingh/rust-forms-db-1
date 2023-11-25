mod connection;


pub use connection::DB_CONN;

use connection::create_db_tables;

pub fn setup_db() {
    create_db_tables().expect("DB did not get created correctly!!!");
}