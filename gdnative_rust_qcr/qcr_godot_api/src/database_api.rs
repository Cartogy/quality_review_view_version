use gdnative::prelude::*;
use gdnative::api::Resource;

use sql_database::db_handler::DBQualityControlHandle;
use sql_database::db::{JobType,Section, Specification};

/* Convert the data obtained from sql_database::db,
 * and have it ready for godot consumption
 */
pub trait ConvertTo<U> {
    fn convert(&self) -> U;
}

impl ConvertTo<JobHeaderData> for JobType {
    fn convert(&self) -> JobHeaderData {
        JobHeaderData { 
            job_id: self.id,
            job_name: self.job_type_name.clone()
        }
    }
}

impl ConvertTo<SectionData> for Section {
    fn convert(&self) -> SectionData {
        SectionData {
            section_id: self.id,
            section_name: self.section_name.clone()
        }
    }
}

/* Because of this conversion, I had to add the section data into Specification.
 * Making it an option as a specification could have no section
 */
impl ConvertTo<SepecificationData> for Specification {
    fn convert(&self) -> SepecificationData {

        let section = {
            match &self.section {
                Some(s) => {
                    Some(
                        SectionData {
                            section_id: s.id,
                            section_name: s.section_name.clone()
                        }
                        )
                },
                None => None
            }
        };

        SepecificationData {
            specification_id: self.id,
            specification_content: self.specification_content.clone(),
            section 
        }
    }
}

/* Storing data to be use in GDScript */
#[derive(NativeClass,ToVariant)]
#[inherit(Resource)]
#[no_constructor]
pub struct JobHeaderData {
    job_id: u64,
    job_name: String,
}

#[derive(NativeClass,ToVariant)]
#[inherit(Resource)]
#[no_constructor]
pub struct SectionData {
    section_id: u64,
    section_name: String
}


#[derive(NativeClass,ToVariant)]
#[inherit(Resource)]
#[no_constructor]
pub struct SepecificationData {
    specification_id: u64,
    specification_content: String,
    section: Option<SectionData>,
}

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct DatabaseAPI {
    db_handle: DBQualityControlHandle
}

#[methods]
impl DatabaseAPI {
    fn new(_owner: &Resource) -> Self {
        Self { db_handle: DBQualityControlHandle::new("database/qcr_database.db".to_string()) }
    }


    #[method]
    pub fn get_all_job_header_info(&mut self) -> Vec<JobHeaderData> {
        let job_types: Vec<JobType> = self.db_handle.get_all_job_types().unwrap();

        let mut job_info = Vec::new();

        for job_type in &job_types {
            let job_i = JobHeaderData { job_id: job_type.id, job_name: job_type.job_type_name.clone() };

            job_info.push(job_i)
        }
        job_info
    }

    fn all_data<T: ConvertTo<U>,U, F: FnMut() -> Vec<T>>(mut f: F) -> Vec<U> {
        // Store all values obtained from function *f*.
        let storage = f();

        let mut vs = Vec::new();

        // Convert these into their values
        for v in storage {
            let d = v.convert();

            vs.push(d);
        }
        vs

    }



    
    #[method]
    pub fn get_all_section_data(&mut self) -> Vec<SectionData> {
        DatabaseAPI::all_data(|| self.db_handle.get_all_sections().unwrap())
    }

    #[method]
    pub fn get_all_specification_data(&mut self) -> Vec<SepecificationData> {
        DatabaseAPI::all_data(|| self.db_handle.get_all_specifications().unwrap())
    }

    #[method]
    pub fn update_section(&mut self, id: u64, content: String) {
        if let Err(_) = self.db_handle.update_section(id, content){
            godot_error!("Failed to update section:");
        }
    }

    #[method]
    pub fn update_specification(&mut self, spec_id: u64, spec_content: String, section_id: u64){
        if let Err(_) = self.db_handle.update_specification_content(spec_id, spec_content) {
            godot_error!("Unable to update specification");
        }
        if let Err(_) = self.db_handle.update_specification_section(spec_id, section_id) {
            godot_error!("Unable to update specification section");
        }
    }
    /*

    fn get_all_job_header(&self) -> Vec<JobHeaderData> {
        self.all_data(self.db_handle.get_all_job_types)
    }

    fn get_all_specification_data(&self) -> Vec<SectionData> {
        let section_datas: Vec<Section> = self.db_handle.get_all_sections();

        let mut sections = Vec::new();

        for section in section_datas {
            let s = SectionData { section_id: section.id, section_name: section.section_name }

            section.push(s)
        }

        sections
    }
    */
}
