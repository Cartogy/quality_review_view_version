use gdnative::prelude::*;
use gdnative::api::Resource;

use sql_database::db_handler::DBQualityControlHandle;
use sql_database::db::{JobType,Section, Specification};

use crate::database_api::{JobHeaderData, ConvertTo};

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct JobDatabaseAPI {
    db_handle: DBQualityControlHandle
}

#[methods]
impl JobDatabaseAPI {
    fn new(_owner: &Resource) -> Self {
        Self { db_handle: DBQualityControlHandle::new("database/qcr_database.db".to_string()) }
    }

    #[method]
    pub fn job_has_specification(&mut self, job_id: u64, specification_id: u64) -> bool {
        let result = self.db_handle.job_has_specification(job_id, specification_id);

        match result {
            Ok(b) => b, // can be true or false.
            Err(_) => false // error? definitely false.
        }
    }

    #[method]
    pub fn job_exists(&mut self, job_type_name: String) -> bool {
        let result = self.db_handle.job_exists(job_type_name);

        match result {
            Ok(b) => b,
            Err(_) => false
        }
    }

    #[method]
    pub fn get_job_type_id(&mut self, job_type_name: String) -> Option<u64> {
        let result = self.db_handle.get_job_type_id(job_type_name);

        match result {
            Ok(val) => Some(val),
            Err(_)  => None
        }
    }

    #[method]
    pub fn get_job_type(&mut self, job_type_id: u64) -> Option<JobHeaderData> {
        let result = self.db_handle.get_job_type(job_type_id);

        match result {
            Ok(job) => Some(job.convert()),
            Err(_)  => None
        }
    }

    #[method]
    pub fn add_job_specification(&mut self, job_type_id: u64, specification_id: u64) {
        let _result = self.db_handle.add_job_specification(job_type_id, specification_id);
    }

    #[method]
    pub fn remove_job_specification(&mut self, job_type_id:u64, specification_id:u64) {
        let _result = self.db_handle.remove_job_spec(job_type_id, specification_id);
    }

    #[method]
    pub fn add_job_type(&mut self, job_type_name: String) {
        let result = self.db_handle.add_job_type(job_type_name);

        if let Err(_) = result  {
            godot_error!("Unable to add job type");
        }
    }

    #[method]
    pub fn update_job_type_name(&mut self, job_type_id: u64, new_name: String) {
        let result = self.db_handle.update_job_type(job_type_id, new_name);

        if let Err(_) = result {
            godot_error!("Unable to update job name");
        }
    }
}
