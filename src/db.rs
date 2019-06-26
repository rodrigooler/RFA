extern crate serde;
extern crate serde_json;

#[macro_use] 
extern crate serde_derive;

use std::fs::File;
use std::io::Read;

use structs::Database;
mod structs;

pub mod {
   pub fn get_db(): Database {
      let mut file = File::open("db.json").unwrap();
      let mut buff = String::new();
      file.read_to_string(&mut buff).unwrap();

      let db: Database = serde_json::from_str(&buff).unwrap();
      db
   }
}