use rusqlite::{params, Connection, Result};
use crate::db::{DBQualityControl,JobType,Section, Specification, JobSpecificationSection};


macro_rules! db_apply {
// Have the function first, to avoid ambiguity with expressions.
    ($f:path, $path:expr, $($x:expr),*) => {
        {
        let conn = Connection::open($path);

        if let Err(e) = conn {
            return Err(vec![e]);
        }

        let conn = conn.unwrap();


        let mut errors = Vec::new();
        
        
        // stores a generic value.
        let result = $f(&conn, // repetedly fill in the arguments.
        $(
            $x,
            
        )*
        );


        // Need to figure out how to close the connection
        let mut conn_open:Option<Connection> = None;
        if let Err((conn,e)) = conn.close() {
            // include the connection to figure how to close it.
            conn_open = Some(conn);
            errors.push(e);
        }


        if errors.len() > 0 {
            Err((conn_open, errors))
        } else {    // 
                    // begin testing result status, to return generic value
            match result {
                Ok(value) => Ok(value),
                Err(e) => {
                    errors.push(e);
                    Err((conn_open, errors))
                }
            }


        }
        }
        
    }
}

pub struct DBQualityControlHandle {
    pub database_path: String,
    // Used to handle failed closing databases.
    connection: Option<Connection>,
}



impl DBQualityControlHandle {
    pub fn new(database_path: String) -> Self {
        Self { database_path, connection: None }
    }

    fn db_apply(&mut self, entry_name: String, f: &dyn Fn(&Connection, String) -> Result<usize>) -> Result<(), Vec<rusqlite::Error>> {
        let conn = Connection::open(&self.database_path);

        if let Err(e) = conn {
            return Err(vec![e]);
        }

        let conn = conn.unwrap();


        let mut errors = Vec::new();

        // Figure out how to have dynamic parameters.
        // That way we only use one function.
        if let Err(e) = f(&conn, entry_name) {
            errors.push(e);
        }

        // Need to figure out how to close the connection
        if let Err((conn,e)) = conn.close() {
            // include the connection to figure how to close it.
            self.connection = Some(conn);
            errors.push(e);
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn job_has_specification(&mut self, job_type_id: u64, specification_id: u64) -> Result<bool, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::job_has_specification, &self.database_path, job_type_id, specification_id);

        self.handle_query(result)
    }

    pub fn job_exists(&mut self, job_type_name: String) -> Result<bool, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::job_exists, &self.database_path, job_type_name);

        self.handle_query(result)
    }

    pub fn add_section(&mut self, section_name: String) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn,errors)) = db_apply!(DBQualityControl::add_section,path, section_name)  {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn add_job_type(&mut self, job_type_name: String) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::add_job_type, path, job_type_name) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
        //self.db_apply(job_type_name, &DBQualityControl::add_job_type)
    }

    pub fn add_specification(&mut self, specification_name: String, section_id: Option<u64>) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::add_specification, path, specification_name, section_id) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn add_additive_section(&mut self, additive_section_name: String) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::add_additive_section, path, additive_section_name) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn add_job_specification(&mut self, job_type_id: u64, specification_id: u64) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::add_job_specification, path, job_type_id, specification_id) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn update_section(&mut self, section_id: u64, section_name: String) -> Result<(), Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::update_section, path,section_id, section_name) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }

    }

    pub fn get_section(&mut self, section_id: u64) -> Result<Section, Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        match db_apply!(DBQualityControl::get_section, path, section_id) {
            Ok(section) => Ok(section), 
            Err((conn, errors)) => {
                if let Some(conn) = conn {
                    self.connection = Some(conn);
                }
                Err(errors)
            }
        }

    }

    pub fn update_job_type(&mut self, job_type_id: u64, job_type_name: String) -> Result<(), Vec<rusqlite::Error>> {

        let path = &self.database_path.clone();

        if let Err((op_conn, errors)) = db_apply!(DBQualityControl::update_job_type, path,job_type_id, job_type_name) {
            if let Some(conn) = op_conn {
                self.connection = Some(conn);
            }

            Err(errors)
        } else {
            Ok(())
        }
    }

    pub fn get_job_type_id(&mut self, job_type_name: String) -> Result<u64, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::get_job_type_id, &self.database_path, job_type_name);

        self.handle_query(result)

    }

    pub fn get_job_type(&mut self, job_type_id: u64) -> Result<JobType, Vec<rusqlite::Error>> {
        let path = &self.database_path.clone();

        match db_apply!(DBQualityControl::get_job_type, path, job_type_id) {
            Ok(job_type) => Ok(job_type), 
            Err((conn, errors)) => {
                if let Some(conn) = conn {
                    self.connection = Some(conn);
                }
                Err(errors)
            }
        }
    }

    pub fn update_specification_content(&mut self, specification_id: u64, specification_name: String) -> Result<usize, Vec<rusqlite::Error>> {

        let result = db_apply!(DBQualityControl::update_specification_content, &self.database_path, specification_id, specification_name) ;
        self.handle_query(result)
    }

    pub fn get_specification(&mut self, specification_id: u64) -> Result<Specification, Vec<rusqlite::Error>> {

        let result = db_apply!(DBQualityControl::get_specification, &self.database_path, specification_id);
        self.handle_query(result)
    }

    pub fn get_all_job_specification(&mut self, job_type_id: u64) -> Result<Vec<JobSpecificationSection>, Vec<rusqlite::Error>> {

        let result = db_apply!(DBQualityControl::get_all_job_specification, &self.database_path, job_type_id);
        self.handle_query(result)
    }

    pub fn get_all_job_types(&mut self) -> Result<Vec<JobType>, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::get_all_job_types, &self.database_path,);
        self.handle_query(result)
    }

    pub fn get_all_sections(&mut self) -> Result<Vec<Section>, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::get_all_sections, &self.database_path,);
        self.handle_query(result)
    }

    pub fn get_all_specifications(&mut self) -> Result<Vec<Specification>, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::get_all_specifications, &self.database_path,);
        self.handle_query(result)
    }

    pub fn update_specification_section(&mut self, specification_id: u64, section_id: u64) -> Result<usize, Vec<rusqlite::Error>> {

        let result = db_apply!(DBQualityControl::update_specification_section, &self.database_path, specification_id, section_id);
        self.handle_query(result)
    }

    pub fn remove_specification(&mut self, specification_id: u64) -> Result<usize, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::remove_specification, &self.database_path, specification_id);
        self.handle_query(result)
    }

    pub fn remove_section(&mut self, section_id: u64) -> Result<usize, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::remove_section,&self.database_path, section_id);
        self.handle_query(result)
    }

    pub fn remove_job_type(&mut self, job_type_id: u64) -> Result<usize, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::remove_job_type, &self.database_path, job_type_id);
        self.handle_query(result)
    }

    pub fn remove_job_spec(&mut self, job_type_id: u64, specification_id: u64) ->Result<usize, Vec<rusqlite::Error>> {
        let result = db_apply!(DBQualityControl::remove_job_spec, &self.database_path, job_type_id, specification_id);
        self.handle_query(result)
    }
    
    // Boiler plate code that deals with a database that failed to closed.
    fn handle_query<T>(&mut self, result: Result<T,(Option<Connection>,Vec<rusqlite::Error>)>) -> Result<T, Vec<rusqlite::Error>> {
        match result {
            Ok(value) => Ok(value),
            Err((conn, errors)) => { 
                if let Some(conn) = conn {
                    self.connection = Some(conn);
                }
                Err(errors)
            }
        }
    }
}
