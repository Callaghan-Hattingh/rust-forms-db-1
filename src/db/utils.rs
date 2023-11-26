/* 
{
    "tablename": "test",
    "columns": [
        {
            "name": "field1",
            "datatype": "BigInt"
        },
        {
            "name": "field2",
            "datatype": "String"
        }
    ]
}
*/

use std::sync::MutexGuard;

use rusqlite::Connection;

pub struct Column {
    pub name: String,
    pub datatype: String,
}

pub struct Table {
    pub tablename: String,
    pub columns: Vec<Column>,    
}

pub struct TableDB<'a> {
    pub conn: &'a MutexGuard<'a, Connection>,
}

impl TableDB<'_> {
    pub fn new<'a>(conn: &'a MutexGuard<Connection>) -> TableDB<'a> {
        TableDB{ conn }
    }

    pub fn dynamic_create_table(&self, table: Table) -> rusqlite::Result<()> {
        let mut stmt = format!("create table if not exists {} (
            id integer primary key,", table.tablename.to_lowercase()); 

        for col in table.columns {
            stmt.push_str(format!("{} {},", col.name, col.datatype).as_str());
        }

        stmt.pop();

        stmt.push(')');

        self.conn.execute(&stmt, ())?;

        Ok(())


    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref DB: Mutex<Connection> = Mutex::new(Connection::open_in_memory().unwrap());
    }

    fn get_db_conn<'a>() -> MutexGuard<'a, Connection> {
        DB.lock().unwrap()
    }

    #[test]
    fn test_dynamic_create_table() {
        let conn = get_db_conn();
        let table_db = TableDB::new(&conn);

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

        table_db.dynamic_create_table(table).unwrap();

        // Verify if table exists
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?").unwrap();
        
        let table_exists: bool = stmt.exists(["test"]).unwrap();

        assert!(table_exists, "Table 'test' should be created.");
    }
}
