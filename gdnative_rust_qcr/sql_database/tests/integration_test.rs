use rusqlite::{params, Connection, Result};
use sql_database::db_handler::DBQualityControlHandle;
use sql_database::db::{JobSpecificationSection, Section,JobType, Specification};

const TEST_DATABASE_PATH: &'static str = "tests/database_test/";

fn create_test_tables(conn: &Connection) -> Result<()> {
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

fn drop_test_tables(conn: &Connection) -> Result<()> {
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

fn populate_section(conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("INSERT INTO section (section_name) VALUES (?1)")?;

    stmt.execute(["Cover page"])?;
    stmt.execute(["Well Data"])?;
    stmt.execute(["Furnace"])?;
    stmt.execute(["Fantasy"])

}

fn populate_job_type(conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("INSERT INTO job_type (job_type_name) VALUES (?1)")?;

    stmt.execute(["Cement"])?;
    stmt.execute(["Horizontal"])?;
    stmt.execute(["Vertical"])
}

fn populate_specification(conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("INSERT INTO specification (specification_content, section_id) VALUES (?1, ?2)")?;

    // For multiple types, use typles instead of an array.
    stmt.execute(("Title",1))?;
    stmt.execute(("Name",1))?;
    stmt.execute(("Location",1))?;

    stmt.execute(("Well Percent", 2))?;
    stmt.execute(("Depth Specified", 2))?;

    stmt.execute(("Door Lock", 3))?;
    stmt.execute(("Pizza Ready", 3))?;
    stmt.execute(("Fire Tested", 3))?;
    stmt.execute(("Homemade", 3))?;

    stmt.execute(("Unicorns Specified", 4))?;
    stmt.execute(("Fairy Tale Bible", 4))?;
    stmt.execute(("Mention of Shrek", 4))
}

fn close_database(conn: Connection) -> Result<()> {
    if let Err((conn,error)) = conn.close() {
        Err(error)
    } else {
        Ok(())
    }
}

fn reset_tables(database_path: &String) -> Result<()> {
        let conn = Connection::open(database_path)?;
        drop_test_tables(&conn)?;

        close_database(conn)
}

fn all_sections(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT * FROM section")?;
    let rows = stmt.query_map([], |row| row.get(1))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
    
}

fn setup_testing_env(database_path: &String) -> Result<()> {
    let conn = Connection::open(database_path)?;

    drop_test_tables(&conn)?;
    create_test_tables(&conn)?;

    close_database(conn)

}

fn show_errors(es: Vec<rusqlite::Error>) {
    for error in es {
        println!("{}",error);
    }
}

fn fail_test(es: Vec<rusqlite::Error>, database_path: &String) {
    show_errors(es);
    reset_tables(database_path);
    assert!(false);

}

fn setup_data(db: &mut DBQualityControlHandle) {
        db.add_job_type("Cement".to_string());
        db.add_job_type("Horizontal".to_string());
        db.add_section("Cover Page".to_string());
        db.add_section("Well Data".to_string());
        db.add_specification("Title".to_string(), Some(1));
        db.add_specification("Subtitle".to_string(), Some(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn table_creation() {
        let conn = Connection::open("tests/database_test/testing_db.db".to_owned());
        if let Err(e) = &conn {
            println!("ERROR opening database: {}",e);
            assert!(false);
        }

        let conn = conn.unwrap();
        if let Err(e) = create_test_tables(&conn) {
            println!("ERROR creating tables: {}",e);
            drop_test_tables(&conn);
            Connection::close(conn);

            assert!(false);
        } else {
            drop_test_tables(&conn);
            Connection::close(conn);
        }

        assert!(true);
    }

    #[test]
    fn table_population() {
    

        let conn = Connection::open("tests/testing_db.db");
        if let Err(e) = &conn {
            println!("ERROR opening database: {}",e);
            assert!(false);
        }

        let conn = conn.unwrap();
        drop_test_tables(&conn);

        if let Err(e) = create_test_tables(&conn) {
            println!("ERROR creating tables: {}",e);
            assert!(false);
        }

        if let Err(e) = populate_section(&conn) {
            println!("ERROR populating sections: {}",e);
            assert!(false);
        }
        if let Err(e) = populate_job_type(&conn) {
            println!("ERROR populating job types: {}",e);
            assert!(false);
        }
        if let Err(e) = populate_specification(&conn) {
            println!("ERROR populating specification: {}",e);
            assert!(false);
        }

        drop_test_tables(&conn);
        Connection::close(conn);
    }

    #[test]
    fn test_add_section() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_section.db".to_string());

        let conn = Connection::open(&db.database_path);
        let conn = conn.unwrap();

        // Prepare tables
        if let Err(e) = setup_testing_env(&db.database_path) {
            fail_test(vec![e], &db.database_path);
        }

        // Insert the section
        if let Err(e) = db.add_section("Cover Page".to_string()) {
            fail_test(e, &db.database_path);
        }

        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT id, section_name FROM section WHERE section_name = ?1",
            ["Cover Page"],
            |row| {
                let id = row.get::<usize,u64>(0)?;
                let section_name = row.get::<usize, String>(1)?;

                Ok((id, section_name))
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/

        reset_tables(&db.database_path);


        // Verify values found in row.
        match id_name_op {
            Ok((id, section)) => {
                assert_eq!(1, id);
                assert_eq!("Cover Page", section);
            },
            Err(e) => {
                println!("ERROR: Failed to add section: {}",e);
                assert!(false);
            }
        }

    }

    #[test]
    fn test_add_job_type() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_job_type.db".to_string());


        setup_testing_env(&db.database_path);

        // Insert the section
        if let Err(e) = db.add_job_type("Cement".to_string()) {
            fail_test(e, &db.database_path);
        }


        let conn = Connection::open(&db.database_path).unwrap();
        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT id, job_type_name FROM job_type WHERE job_type_name = ?1",
            ["Cement"],
            |row| {
                let id = row.get::<usize,u64>(0)?;
                let section_name = row.get::<usize, String>(1)?;

                Ok((id, section_name))
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/

        // Verify values found in row.
        match id_name_op {
            Ok((id, section)) => {
                assert_eq!(1, id);
                assert_eq!("Cement", section);
            },
            Err(e) => {
                println!("ERROR: Failed to add cement: {}",e);
                assert!(false);
            }
        }


        drop_test_tables(&conn);
        close_database(conn);
    }

    #[test]
    fn test_add_specification() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_spec.db".to_string());

        let conn = Connection::open(&db.database_path);
        let conn = conn.unwrap();

        // Prepare tables
        drop_test_tables(&conn);
        create_test_tables(&conn);

        // Insert the section
        db.add_section("Cover Page".to_string());
        //
        if let Err(e) = db.add_specification("Title".to_string(), Some(1)) {
            show_errors(e);
            assert!(false);
        }

        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT id, specification_content, section_id FROM specification WHERE specification_content = ?1",
            ["Title"],
            |row| {
                let id = row.get::<usize,u64>(0)?;
                let specification_content = row.get::<usize, String>(1)?;
                let section_id = row.get::<usize,u64>(2)?;

                Ok((id, specification_content, section_id))
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/


        // Verify values found in row.
        match id_name_op {
            Ok((id, spec, section_id)) => {
                assert_eq!(1, id);
                assert_eq!("Title", spec);
                assert_eq!(1, section_id);
            },
            Err(e) => {
                println!("ERROR: Failed to add Title specification: {}",e);
                assert!(false);
            }
        }


        drop_test_tables(&conn);
        close_database(conn);
    }

    #[test]
    fn test_add_specification_null() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_spec_null.db".to_string());

        let conn = Connection::open(&db.database_path);
        let conn = conn.unwrap();

        // Prepare tables
        drop_test_tables(&conn);
        create_test_tables(&conn);

        // Insert the section
        db.add_section("Cover Page".to_string());
        //
        if let Err(e) = db.add_specification("Title".to_string(), None) {
            show_errors(e);
            assert!(false);
        }

        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT id, specification_content, section_id FROM specification WHERE specification_content = ?1",
            ["Title"],
            |row| {
                let id = row.get::<usize,u64>(0)?;
                let specification_content = row.get::<usize, String>(1)?;
                
                // Value can be null or not.
                let section_id  = row.get_ref_unwrap(2).as_i64_or_null();

                Ok((id, specification_content, section_id))
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/


        // Verify values found in row.
        match id_name_op {
            Ok((id, spec,Ok(None))) => {
                assert_eq!(1, id);
                assert_eq!("Title", spec);
                assert!(true);
            },
            Ok(_) => assert!(false),
            Err(e) => {
                println!("ERROR: Failed to add Title specification: {}",e);
                assert!(false);
            }
        }


        drop_test_tables(&conn);
        close_database(conn);
    }


    #[test]
    fn test_add_additive_section() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_additive.db".to_string());

        let conn = Connection::open(&db.database_path);
        let conn = conn.unwrap();

        // Prepare tables
        drop_test_tables(&conn);
        create_test_tables(&conn);

        // Insert the section
        db.add_section("Cover Page".to_string());
        //
        if let Err(e) = db.add_additive_section("Cover Page".to_string()) {
            show_errors(e);
            assert!(false);
        }

        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT section_id FROM additive_section WHERE section_id = ?1",
            [1],
            |row| {
                let id = row.get::<usize,u64>(0)?;
                
                // Value can be null or not.
                let section_id  = row.get(0);

                section_id
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/


        // Verify values found in row.
        match id_name_op {
            Ok(id) => {
                assert_eq!(1, id);
                assert!(true);
            },
            Err(e) => {
                println!("ERROR: Failed to add Title specification: {}",e);
                assert!(false);
            }
        }


        drop_test_tables(&conn);
        close_database(conn);
    }

    #[test]
    fn test_add_job_specification() {
        let mut db = DBQualityControlHandle::new("tests/testing_db_job_spec.db".to_string());

        let conn = Connection::open(&db.database_path);
        let conn = conn.unwrap();

        // Prepare tables
        drop_test_tables(&conn);
        create_test_tables(&conn);

        // Insert the section
        if let Err(e) = db.add_section("Cover Page".to_string()) {
            println!("ERROR in Job specification");
            show_errors(e);
                assert!(false);
        }
        if let Err(e) = db.add_job_type("Cement".to_string()) {
            println!("ERROR in adding a job type");
            show_errors(e);
                assert!(false);
        }
        if let Err(e) = db.add_specification("Title".to_string(), None) {
            println!("ERROR in adding a specification:");
            show_errors(e);
                assert!(false);
        }

        if let Err(e) = db.add_job_specification(1,1) {
            println!("ERROR in adding a job specification:");
            show_errors(e);
            assert!(false);
        }

        // Check the row was successful.
        let id_name_op = conn.query_row(
            "SELECT job_type_id, specification_id FROM job_specification WHERE job_type_id = ?1 AND specification_id = ?2",
            [1,1],
            |row| {
                let job_type_id = row.get::<usize,u64>(0)?;
                let specification_id = row.get::<usize, u64>(1)?;
                
                // Value can be null or not.

                Ok((job_type_id, specification_id))
            });

        // Show all names in section
        /* for v in all_sections(&conn).unwrap() {
            println!("{}",v);
        }*/


        // Verify values found in row.
        match id_name_op {
            Ok((job_type_id, spec_id)) => {
                assert_eq!(1, job_type_id);
                assert_eq!(1, spec_id);
                assert!(true);
            },
            Err(e) => {
                println!("ERROR: Failed to add Title specification: {}",e);
                assert!(false);
            }
        }


        drop_test_tables(&conn);
        close_database(conn);
    }

    #[test]
    fn update_section() {
        let mut db = DBQualityControlHandle::new("test_db_update_section.db".to_string());

        setup_testing_env(&db.database_path);

        db.add_section("Cover Page".to_string());

        let section_update = String::from("Cover Page - Header");

        /*
        if let Err((conn, errors)) = db.update_section(1, section_update) {
            show_errors(errors);
            assert!(false);
        }*/

        if let Err(e) = db.update_section(1,section_update) {
            println!("ERROR updating section");
            show_errors(e);
            assert!(false);
        }
        let result = db.get_section(1);
        /*
        let conn = Connection::open(&db.database_path).unwrap();

        let res_section = conn.query_row(
            "SELECT section_name FROM section WHERE id = ?1",
            [1],
            |row| row.get::<usize,String>(0));

        conn.close();
        */

        if let Ok(section) = result {
            assert_eq!("Cover Page - Header".to_string(), section.section_name);
        } else {
            assert!(false);
        }

    }

    #[test]
    fn update_job_type() {
        let mut db = DBQualityControlHandle::new("test_db_update_job_type.db".to_string());

        setup_testing_env(&db.database_path);
        db.add_job_type("Cement".to_string());

        let job_type_update = String::from("Cement Elongation");
        db.update_job_type(1,job_type_update);

        let result = db.get_job_type(1);

        /*
        if let Err((conn, errors)) = db.update_job_type(1, job_type_update) {
            show_errors(errors);
            assert!(false);
        }
        */



        if let Ok(job_type_name) = result {
            assert_eq!("Cement Elongation".to_string(), job_type_name.job_type_name);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn update_specification_content() {
        let mut db = DBQualityControlHandle::new("test_db_update_specification_content.db".to_string());

        setup_testing_env(&db.database_path);

        // Prepare tables
        db.add_job_type("Cement".to_string());
        db.add_section("Cover Page".to_string());
        db.add_specification("Title".to_string(), Some(1));

        db.update_specification_content(1,"Title - Header".to_string());

        let result = db.get_specification(1);

        match  result {
            Ok(specification) => { 
                assert_eq!("Title - Header", specification.specification_content);
            },
            Err(e) => {
                show_errors(e);
                assert!(false);
            }
        }

    }

    #[test]
    fn update_specification_section() {
        let mut db = DBQualityControlHandle::new("test_db_update_specification_section.db".to_string());

        setup_testing_env(&db.database_path);

        db.add_job_type("Cement".to_string());
        db.add_section("Cover Page".to_string());
        db.add_section("Well Data".to_string());
        db.add_specification("Title".to_string(), Some(1));

        db.update_specification_section(1,2);

        let result = db.get_specification(1);

        if let Ok(specification) = result {
            match specification.section {
                Some(s) => {
                    assert_eq!(2, s.id);
                },
                _ => {assert!(false);}
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn remove_specification() {
        let mut db = DBQualityControlHandle::new("tests/database_test/test_db_remove_specification.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        db.remove_specification(1);

        let result = db.get_specification(1);

        if let Err(_) = result {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn remove_section() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_remove_section.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        db.remove_section(1);

        let result = db.get_section(1);

        if let Err(_) = result {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn remove_job_type() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_remove_job_type.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        db.remove_job_type(1);

        let result = db.get_job_type(1);

        if let Err(_) = result {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn all_job_specification() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_all_job_specification.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        db.add_job_specification(1,1);
        db.add_job_specification(1,2);

        let result = db.get_all_job_specification(1);

        let section_one = Section::new(1,"Cover Page".to_string());

        let expected_job_spec_one = JobSpecificationSection::new(1,"Cement".to_string(),Some(section_one.clone()),1,"Title".to_string());
        let expected_job_spec_two = JobSpecificationSection::new(1,"Cement".to_string(),Some(section_one.clone()),2,"Subtitle".to_string());

        match result {
            Ok(job_spec) => {
                if job_spec.len() != 2 {
                    assert!(false);
                } else {
                    let q_0 = &job_spec[0];
                    let q_1 = &job_spec[1];

                    assert_eq!(expected_job_spec_one, *q_0);
                    assert_eq!(expected_job_spec_two, *q_1);
                    
                }
            },
            Err(e) => assert!(false),
        }
        
    }

    #[test]
    fn get_all_job_types() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_all_job_types.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        let result = db.get_all_job_types();

        let job1 = JobType::new(1,"Cement".to_string());
        let job2 = JobType::new(2, "Horizontal".to_string());

        let expected_jobs = vec![job1,job2];

        match result {
            Ok(jobs) => {
                assert_eq!(expected_jobs, jobs);
            },
            Err(e) => {
                show_errors(e);
                assert!(false);
            }
        }
    }

    #[test]
    fn get_all_sections() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_all_sections.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        let result = db.get_all_sections();

        let section1 = Section::new(1,"Cover Page".to_string());
        let section2 = Section::new(2,"Well Data".to_string());

        let expected_sections = vec![section1, section2];

        match result {
            Ok(sections) => {
                assert_eq!(expected_sections, sections);
            },
            Err(e) => {
                show_errors(e);
                assert!(false);
            }
        }
    }

    #[test]
    fn get_all_specifications() {
        let mut db = DBQualityControlHandle::new("tests/database_test/db_all_specs.db".to_string());

        setup_testing_env(&db.database_path);
        setup_data(&mut db);

        let result = db.get_all_specifications();

        let section1 = Section::new(1,"Cover Page".to_string());

        let spec1 = Specification::new(1,"Title".to_string(),Some(section1.clone()));
        let spec2 = Specification::new(2,"Subtitle".to_string(),Some(section1.clone()));

        let expected_specs = vec![spec1, spec2];

        match result {
            Ok(specs) => {
                assert_eq!(expected_specs, specs);
            },
            Err(e) => {
                show_errors(e);
                assert!(false);
            }
        }
    }



}
