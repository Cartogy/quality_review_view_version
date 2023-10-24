use rusqlite::{Connection, Result};

#[derive(Debug, PartialEq)]
pub struct JobType {
    pub id: u64,
    pub job_type_name: String,
}

impl JobType {
    pub fn new(id: u64, job_type_name: String) -> Self {
        Self { id, job_type_name }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Section {
    pub id: u64,
    pub section_name: String,
}

impl Section {
    pub fn new(id: u64, section_name: String) -> Self {
        Section { id, section_name }
    }
}

#[derive(Debug, PartialEq)]
pub struct Specification {
    pub id: u64,
    pub specification_content: String,
    pub section: Option<Section>,
}

impl Specification {
    pub fn new(id: u64, specification_content: String, section: Option<Section>) -> Self {
        Self {
            id, specification_content, section
        }
    }
}

struct JobSpecification {
    job_type_id: u64,
    specification_id: u64,
}

// Maps the ids to their respective names/content. 
#[derive(Debug)]
pub struct JobSpecificationSection {
    pub job_type_id: u64,
    pub job_name: String,

    pub section: Option<Section>,

    pub specification_id: u64,
    pub specification_content: String
}

impl PartialEq for JobSpecificationSection {
    fn eq(&self, other: &Self) -> bool {
        let valid_section = {
            match (&self.section, &other.section) {
                (Some(s),Some(ss)) => {
                    s == ss
                },
                (None,None) => true,
                _ => false
            }
        };

        self.job_type_id == other.job_type_id && 
            self.job_name == other.job_name && 
            self.specification_id == other.specification_id && 
            self.specification_content == other.specification_content && 
            valid_section
    }
}

impl JobSpecificationSection {
    pub fn new(job_type_id: u64, job_name: String, section: Option<Section>, specification_id: u64, specification_content: String) -> Self {
        Self {
            job_type_id,
            job_name,
            section,
            specification_id,
            specification_content
        }
    }
}

pub struct DBQualityControl;

/*
 * There was a pattern in manipulation the database via the Handler.
 *
 */
/*
macro_rules! table_manipulation {
    ($f:path, $path:expr, $($x:expr),+) => {

        match add_entry!($f, $path, $( $x, )+) {
            Ok(job_type) => Ok(job_type), 
            Err((conn, errors)) => {
                if let Some(conn) = conn {
                    self.connection = Some(conn);
                }
                Err(errors)
            }
        }

    }
}
*/


/* This structure is responsible for dealing with connections */
/* The handle struct manages closing errors */
impl DBQualityControl {

    pub fn job_has_specification(conn: &Connection, job_type_id: u64, specification_id: u64) -> Result<bool> {

        match conn.query_row("SELECT job_type_id specification_id FROM job_specification WHERE job_type_id = ?1 AND specification_id = ?2",[job_type_id,specification_id],
                       |_row| Ok(true)) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    pub fn job_exists(conn: &Connection, job_type_name: String) -> Result<bool> {
        match conn.query_row("SELECT id, job_type_name FROM job_type WHERE job_type_name = ?1",[job_type_name],
                       |_row| Ok(true)) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }

    }


    pub fn add_section(conn: &Connection, section_name: String) -> Result<usize> {

       // Create new scope to have statement borrow *conn* without having to create a new function.
       let insert_status = {
           let mut stmt = conn.prepare("INSERT INTO section (section_name) VALUES (?1)")?;
           stmt.execute([section_name])

       };

       insert_status

    }

    pub fn add_additive_section(conn: &Connection, additive_section_name: String) -> Result<usize> {
        conn.execute("PRAGMA foreign_keys = 1", [])?;

        let op_section_id = {

            let mut stmt = conn.prepare("SELECT id FROM section WHERE section_name = ?")?;
            let section_id = stmt.query_row([additive_section_name], |row| row.get::<usize,u64>(0));
            section_id

        };

        match op_section_id {
            Ok(id) => {
                conn.execute("INSERT INTO additive_section (section_id) VALUES (?1)",[id])
            },
            Err(e) => { Err::<usize, rusqlite::Error>(e) }
        }

    }

    pub fn add_job_type(conn: &Connection,job_type_name: String) -> Result<usize> {

        {
           let mut stmt = conn.prepare("INSERT INTO job_type (job_type_name) VALUES (?1)")?;
           stmt.execute([job_type_name])
        }

    }


    pub fn add_specification(conn: &Connection, specification_name: String, section_id: Option<u64>) -> Result<usize> {

        // set constraints to satisfy foreign key.
        conn.execute("PRAGMA foreign_keys = 1", [])?;

        {
           let mut stmt = conn.prepare("INSERT INTO specification (specification_content, section_id) VALUES (?1, ?2)")?;
           //
           // The section id can be null or not.
           // We specify this using the None value.
           match section_id {
               Some(s) => {stmt.execute((specification_name, s))},
               None    => {stmt.execute((specification_name, rusqlite::types::Null))},
           }
        }


    }

    pub fn add_job_specification(conn: &Connection, job_type_id: u64, specification_id: u64) -> Result<usize> {

        conn.execute("PRAGMA foreign_keys = 1", [])?;
        {
            let mut stmt = conn.prepare("INSERT INTO job_specification (job_type_id, specification_id) VALUES (?1, ?2)")?;
            stmt.execute((job_type_id, specification_id))

        }
    }

    pub fn update_section(conn: &Connection, section_id: u64, section_name: String) -> Result<usize> {
        {
            let mut stmt = conn.prepare("UPDATE section SET section_name = ?2 WHERE id = ?1")?;
            stmt.execute((section_id, section_name))

        }
    }

    pub fn update_job_type(conn: &Connection, job_type_id: u64, job_type_name: String) -> Result<usize> {
        {
            let mut stmt = conn.prepare("UPDATE job_type SET job_type_name = ?2 WHERE id = ?1")?;
            stmt.execute((job_type_id, job_type_name))
        }
    }

    pub fn update_specification_content(conn: &Connection, specification_id: u64, specification_name: String) -> Result<usize> {
        {
            let mut stmt = conn.prepare("UPDATE specification SET specification_content = ?2 WHERE id = ?1")?;
            stmt.execute((specification_id, specification_name))
        }
    }

    pub fn update_specification_section(conn: &Connection, specification_id: u64, section_id: u64) -> Result<usize> {
        {
            let mut stmt = conn.prepare("UPDATE specification SET section_id = ?2 WHERE id = ?1")?;
            stmt.execute((specification_id, section_id))
        }
    }

    pub fn remove_specification(conn: &Connection, specification_id: u64) -> Result<usize> {
        {
            let mut stmt = conn.prepare("DELETE FROM specification WHERE id = ?1")?;
            stmt.execute([specification_id])
        }
    }

    pub fn remove_section(conn: &Connection, section_id: u64) -> Result<usize> {
        {
            let mut stmt = conn.prepare("DELETE FROM section WHERE id = ?1")?;
            stmt.execute([section_id])
        }
    }

    pub fn remove_job_type(conn: &Connection, job_type_id: u64) -> Result<usize> {
        {
            let mut stmt = conn.prepare("DELETE FROM job_type WHERE id = ?1")?;
            stmt.execute([job_type_id])
        }
    }

    pub fn remove_job_spec(conn: &Connection, job_type_id: u64, specification_id: u64) -> Result<usize> {
        {
            let mut stmt = conn.prepare("DELETE FROM job_specification WHERE job_type_id = ?1 AND specification_id = ?2")?;
            stmt.execute([job_type_id,specification_id])
        }
    }

    pub fn get_section(conn: &Connection, section_id: u64) -> Result<Section> {
        conn.query_row("SELECT id, section_name FROM section WHERE id = ?1",
                       [section_id],
                       |row| Ok(
                                Section {
                                    id: row.get(0)?,
                                    section_name: row.get(1)?
                                }
                           )
                       )
    }

    pub fn get_job_type_id(conn: &Connection, job_type_content: String) -> Result<u64> {
        conn.query_row("SELECT id FROM job_type WHERE job_type_name = ?1", [job_type_content],
                       |row| row.get(0))
    }

    pub fn get_job_type(conn: &Connection, job_type_id: u64) -> Result<JobType> {
        conn.query_row("SELECT id, job_type_name FROM job_type WHERE id = ?1",
                       [job_type_id],
                       |row| Ok(
                            JobType {
                                id: row.get(0)?,
                                job_type_name: row.get(1)?
                            }
                           )
                       )
    }

    pub fn get_all_job_types(conn: &Connection) -> Result<Vec<JobType>> {
        {
            let mut stmt = conn.prepare("SELECT id, job_type_name FROM job_type")?; 
            let rows = stmt.query_map([], |row| Ok(JobType { 
                                                        id: row.get(0)?, 
                                                        job_type_name: row.get(1)?
                                                    })
                                      )?;

            let mut job_types = Vec::new();
            for row in rows {
                job_types.push(row?);
            }

            Ok(job_types)
        }
    }

    pub fn get_all_sections(conn: &Connection) -> Result<Vec<Section>> {
        {
            let mut stmt = conn.prepare("SELECT id, section_name FROM section")?;
            let rows = stmt.query_map([], |row| Ok(
                                                    Section {
                                                        id: row.get(0)?,
                                                        section_name: row.get(1)?
                                                    }
                                                ))?;

            let mut sections = Vec::new();
            for row in rows {
                sections.push(row?);
            }

            Ok(sections)
        }
    }

    pub fn get_specification(conn: &Connection, specification_id: u64) -> Result<Specification> {
        conn.query_row("SELECT specification.id, specification_content, section_id, section_name FROM specification INNER JOIN section ON section.id = section_id WHERE specification.id = ?1",
                       [specification_id],
                       |row| {
                           // Build section
                           let section = match (row.get_ref(2)?.as_i64_or_null()?, row.get_ref(3)?.as_str_or_null()?) {
                               (Some(s_id),Some(s_name)) => {
                                    Some(Section{
                                        id: u64::try_from(s_id).unwrap(),
                                        section_name: s_name.to_string()
                                    })
                               },
                               _ => None
                           };
                           // Build Spec
                            Ok(
                               Specification {
                                   id: row.get(0)?,
                                   specification_content: row.get(1)?,
                                   section
                               }
                           )

                           }                    
                       )
    }

    pub fn get_all_specifications(conn: &Connection) -> Result<Vec<Specification>> {
        {
            let mut stmt = conn.prepare("SELECT specification.id, specification_content, section.id, section_name FROM specification INNER JOIN section ON section.id = section_id")?;

            let rows = stmt.query_map([],|row| {
                let section = DBQualityControl::row_section(&row)?;

                Ok(
                    Specification {
                        id: row.get(0)?,
                        specification_content: row.get(1)?,
                        section

                    }
                    )

            })?;

            let mut specs = Vec::new();
            for row in rows {
                specs.push(row?);
            }
            Ok(specs)
        }
    }

    fn row_section(row: &rusqlite::Row) -> Result<Option<Section>> {
           let section = match (row.get_ref(2)?.as_i64_or_null()?, row.get_ref(3)?.as_str_or_null()?) {
               (Some(s_id),Some(s_name)) => {
                    Ok(Some(Section{
                        id: u64::try_from(s_id).unwrap(),
                        section_name: s_name.to_string()
                    }))
               },
               _ => Ok(None)
           };
           section
    }

    pub fn get_all_job_specification(conn: &Connection, job_type_id: u64) -> Result<Vec<JobSpecificationSection>> {
        {
            let mut stmt = conn.prepare("SELECT job_type_id, job_type_name, section_id, section_name, specification_id, specification_content 
                                         FROM job_specification
                                         INNER JOIN specification ON specification_id = specification.id
                                         INNER JOIN job_type ON job_type_id = job_type.id
                                         INNER JOIN section ON section_id = section.id
                                         WHERE job_type_id = ?1
                                         ")?;

            let rows = stmt.query_map([job_type_id], |row| {
                   let section = match (row.get_ref(2)?.as_i64_or_null()?, row.get_ref(3)?.as_str_or_null()?) {
                       (Some(s_id),Some(s_name)) => {
                            Some(Section{
                                id: u64::try_from(s_id).unwrap(),
                                section_name: s_name.to_string()
                            })
                       },
                       _ => None
                   };

                Ok(
                        JobSpecificationSection {
                            job_type_id: row.get(0)?,
                            job_name: row.get(1)?,
                            section,
                            specification_id: row.get(4)?,
                            specification_content: row.get(5)?,

                        }
                    )}
                )?;

            let mut job_specs: Vec<JobSpecificationSection> = Vec::new();

            for row in rows {
                job_specs.push(row?);
            }

            Ok(job_specs)
        }
    }

}


