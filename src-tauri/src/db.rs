use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Creating the Schema for the table
#[derive(Queryable, Selectable)]
pub struct Record {
    pub id: i32,
    pub rec_key: String,
    pub rec_value: String,
}

/* Creating a table records with the schema */
table! {
    records (id) {
        id -> Integer,
        rec_key -> Text,
        rec_value -> Text,
    }
}

pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
    create_table();

    // run_migrations();
}

// create records table
//

// fn run_migrations() {
//     let mut connection = establish_connection();
//     connection.run_pending_migrations(MIGRATIONS).unwrap();
// }

fn establish_connection() -> SqliteConnection {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        println!("Creating directory {}", db_dir.display());
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/orion/database.sqlite"
}

// table create with diesel
//
fn create_table() {
    let mut connection = establish_connection();
    let _ = diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS records (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        rec_key TEXT NOT NULL,
        rec_value TEXT NOT NULL
    )",
    )
    .execute(&mut connection);
}

// table insert with diesel
pub fn insert_key_value(key: &str, value: &str) {
    let mut connection = establish_connection();
    // let cloned_connection = connection.clone();
    let _ = diesel::sql_query("INSERT INTO records (rec_key, rec_value) VALUES (?1, ?2)")
        .bind::<diesel::sql_types::Text, _>(key)
        .bind::<diesel::sql_types::Text, _>(value)
        .execute(&mut connection)
        .expect("Error inserting record");

    // let mut select_connection = establish_connection();

    let _results = records::table
        .select(records::all_columns)
        .load::<Record>(&mut connection)
        .expect("Error loading records");
    // println!("Displaying {} records", results.len());
    // for record in results {
    //     println!("{}: {} - {}", record.id, record.rec_key, record.rec_value);
    // }
}

pub fn delete_key_value(key: &str) -> Option<String> {
    let mut connection = establish_connection();
    let _ = diesel::sql_query("DELETE FROM records WHERE rec_key = ?1")
        .bind::<diesel::sql_types::Text, _>(key)
        .execute(&mut connection);

    let _results = records::table
        .select(records::all_columns)
        .load::<Record>(&mut connection)
        .expect("Error loading records");
    // println!("Displaying {} records", results.len());
    // for record in results {
    //     println!("{}: {} - {}", record.id, record.rec_key, record.rec_value);
    // }

    None
}

pub fn load_key_value_pairs() -> Vec<(String, String)> {
    let mut connection = establish_connection();
    let results = records::table
        .select(records::all_columns)
        .load::<Record>(&mut connection)
        .expect("Error loading records");

    let result: Vec<(String, String)> = results
        .iter()
        .map(|record| (record.rec_key.to_string(), record.rec_value.to_string()))
        .collect();

    result
}
