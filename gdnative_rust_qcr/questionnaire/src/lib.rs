use std::error::Error;

pub mod data;
pub mod job;
pub mod questionnaire;

pub trait CSVWrite {
    fn write_csv(&self, file_path: String) -> Result<(), Box<dyn Error>>;
}
