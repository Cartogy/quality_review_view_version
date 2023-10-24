
use std::env;
use std::io;
use std::process;
use std::error::Error;
use std::fs::File;

use serde::Deserialize;
use csv::Reader;

use rusqlite::Connection;

#[derive(Debug,Deserialize)]
struct Row {
    Item: String,
    Heading: String,
    Content: String,
}

// Used to return the OS file_path.
use std::ffi::OsString;
fn main() {
    if let Err(e) = run() {
        println!("{}",e);
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;

    let mut rdr = csv::Reader::from_reader(file);

    let mut records = vec![];

    for result in rdr.deserialize() {
        let row: Row = result?;
        records.push(row);
    }


    fill_database(records);
    /*for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        for r in &record {
            //println!("{}", r);
        }
    }
    */

    Ok(())
}

fn fill_database(records: Vec<Row>) {
    let conn = Connection::open("qcr_database.db").unwrap();
    setup_database(&conn);
    
    for record in records {
        conn.execute("INSERT INTO section (section_name) VALUES (?1)",[record.Heading.clone()]);

        let op_id = get_section_id(&conn, record.Heading.clone());
        match op_id {
            Some(id) => {
                conn.execute("INSERT INTO specification (specification_content, section_id) VALUES (?1, ?2)",(record.Content, id));
            },
            None    => {
                conn.execute("INSERT INTO specification (specification_content, section_id) VALUES (?1, ?2)",(record.Content, rusqlite::types::Null));
            }
        }

    }

    conn.close();
}

fn get_section_id(conn: &Connection, section_name: String) -> Option<u64> {
    match conn.query_row("SELECT id FROM section WHERE section_name = ?1",[section_name], |row| row.get(0)) {
        Ok(id) => Some(id),
        Err(e) => None,
    }


}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn setup_database(conn: &Connection) -> Result<(), rusqlite::Error>{

    drop_database(&conn);

    conn.execute_batch(
        "BEGIN;
        CREATE TABLE IF NOT EXISTS section (
            id INTEGER PRIMARY KEY,
            section_name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS job_type (
            id INTEGER PRIMARY KEY,
            job_type_name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS additive_section (
            section_id INTEGER,
            FOREIGN KEY (section_id)
                REFERENCES section (id)
                    ON DELETE CASCADE
                    ON UPDATE NO ACTION
        );
        CREATE TABLE IF NOT EXISTS specification (
            id INTEGER PRIMARY KEY,
            specification_content TEXT NOT NULL UNIQUE,
            section_id INTEGER,
            FOREIGN KEY (section_id)
                REFERENCES section (id)
                    ON DELETE SET NULL
                    ON UPDATE NO ACTION
        );
        CREATE TABLE IF NOT EXISTS job_specification (
            job_type_id INTEGER,
            specification_id INTEGER,
            PRIMARY KEY (job_type_id, specification_id),
            FOREIGN KEY (job_type_id)
                REFERENCES job_type (id)
                    ON DELETE CASCADE
                    ON UPDATE NO ACTION,
            FOREIGN KEY (specification_id)
                REFERENCES specification (id)
                    ON DELETE CASCADE
                    ON UPDATE NO ACTION
        );
        COMMIT;")
}

fn drop_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        BEGIN;
        DROP TABLE IF EXISTS section;
        DROP TABLE IF EXISTS additive_section;
        DROP TABLE IF EXISTS job_type;
        DROP TABLE IF EXISTS specification;
        DROP TABLE IF EXISTS job_specification;
        COMMIT;
        "
        )
}
