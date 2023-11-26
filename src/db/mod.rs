mod connection;
mod truck;
mod load;
mod utils;


pub use connection::DB_CONN;

use connection::create_db_tables;
pub use utils::{Table, Column, TableDB};

pub fn setup_db() {
    create_db_tables().expect("DB did not get created correctly!!!");
}