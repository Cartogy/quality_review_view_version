/*
 * QCReport contains the data that is to be manipulate by Godot's QCReportView Control Node
 *
 * All functions update the state of the data.
 */

use gdnative::prelude::*;
use gdnative::api::Resource;

use questionnaire::questionnaire::Questionnaire;
//use crate::questionnaire_data::QuestionnaireData;
use questionnaire::questionnaire::{UnitForm, QuestionStatus, PDFable};
use questionnaire::data::{Id, Question, Section};
use questionnaire::job::Job;

use sql_database::db_handler::DBQualityControlHandle;

use std::collections::HashMap;
use sql_database::db::{JobSpecificationSection};
use sql_database::*;
use plotting::PlotData;

use questionnaire::CSVWrite;


struct GDQuestionStatus(QuestionStatus);

impl ToVariant for GDQuestionStatus {
    fn to_variant(&self) -> Variant {
        match self.0 {
            QuestionStatus::OK => 0.to_variant(),
            QuestionStatus::NO => 1.to_variant(),
            QuestionStatus::NA => 2.to_variant()
        }
    }
}


impl FromVariant for GDQuestionStatus {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let result =  i64::from_variant(variant)?;
        match result {
            0 => Ok(GDQuestionStatus(QuestionStatus::OK)),
            1 => Ok(GDQuestionStatus(QuestionStatus::NO)),
            2 => Ok(GDQuestionStatus(QuestionStatus::NA)),
            _ => Err(FromVariantError::UnknownEnumVariant {
                variant: "i64".to_owned(),
                expected: &["0","1","2"],
            }),
        }
    }
}

/* Input generate from user
 *
 * Godot Input -> JobQuery -> rust
 */
#[derive(FromVariant)]
struct JobQuery {
    job_name: String,
    job_id: u64,
    additional_sections: Vec<String>,
}

#[derive(NativeClass, FromVariant)]
#[no_constructor]
struct HeaderInfo {
    engineer: String,
    job: String,
    day: i64,
    month: i64,
    year: i64,
}

// Contains the data for the current instance of the report.
#[derive(NativeClass)]
#[inherit(Resource)]
pub struct QCReport {
    pub questionnaire_data: Option<Questionnaire>,
    header_info: Option<HeaderInfo>,
    plot_data: Option<PlotData>

}

#[methods]
impl QCReport {
    fn new(_owner: &Resource) -> Self {

        QCReport { questionnaire_data: None::<Questionnaire>,
        header_info: None::<HeaderInfo>,
        plot_data: None::<PlotData> }
    }

    #[method]
    fn write_csv(&self, form_file_path: String, plot_file_path: String) {
        match (&self.questionnaire_data, &self.plot_data) {
            (None, None) => godot_error!("No data to write"),
            (_, None) => godot_error!("No plot data"),
            (None, _) => godot_error!("No questionnaire data"),
            (Some(q), Some(plot)) => {
                q.write_csv(form_file_path);
                plot.write_csv(plot_file_path);
            }
        }
    }

    #[method]
    fn build_report(&mut self, header_info: HeaderInfo, query: JobQuery) {
        let result = self.acquire_questionnaire_data(query);

        match result {
            Ok(ques) => {
                self.header_info = Some(header_info);
                //godot_print!("Questionnaire: {:?}", &ques);
                self.questionnaire_data = Some(ques); 
            },
            Err(e) => { godot_error!("Failed to acquire data: {}",e) 
            }
        }
    }

    #[method]
    fn build_plot(&mut self) {
        match &self.questionnaire_data {
            Some(qs) => {
                let mut plot_data = PlotData::new();

                let mut db_handle = DBQualityControlHandle::new("database/qcr_database.db".to_string());
                // Build data for plot
                for (form_id, form) in qs.all_forms() {
                    let section_id = form.get_section_id();
                    let section: db::Section = db_handle.get_section(section_id.primitive()).unwrap();

                    godot_print!("From status from section {}: {}", section.section_name, form.get_status());
                    
                    match form.get_status() {
                        QuestionStatus::OK => plot_data.increment_yes(section.section_name),
                        QuestionStatus::NO => plot_data.increment_no(section.section_name),
                        _ => {},
                    }

                }

                self.plot_data = Some(plot_data);
            },
            None => {
                godot_error!("No questionnaire data available");
            }
        }
    }

    #[method]
    fn draw_plot(&self) {
        if let None = self.plot_data {
            godot_error!("No Plot data available");
            return;
        }

        match &self.plot_data {
            Some(plot_data) => plot_data.make_plot(),
            None => { godot_error!("No Plot data") }
        }
    }

    // Interfaces with the rust sqlite database.
    fn acquire_questionnaire_data(&self, query: JobQuery) -> Result<Questionnaire, &'static str> {
        // 1. Get access to database
        let mut db_handle = DBQualityControlHandle::new("database/qcr_database.db".to_string());
        let result = db_handle.get_all_job_specification(query.job_id);

        // Add question if found
        match result {
            Ok(job_specs) => {
                // At this points we have all the job specifications.
                //godot_print!("Job Specifications - {:?}", job_specs);

                let op_jobs = QCReport::job_query_to_job_questionnaire(job_specs);


                match op_jobs {
                    Ok(mut jobs_hash) => {
                        match jobs_hash.remove(&query.job_id) {
                            Some(job) => { 
                                //godot_print!("{:?}",job);
                                Ok(Questionnaire::new(job))
                            },
                            None     => Err("No job found with id"),
                        }
                    },
                    Err(e) => Err("Failed to convert database to run-time data")
                }
            },
            Err(e) => Err("Failed to access database job specifications")
        }
    }

    fn set_job(&mut self, questionnaire: Questionnaire) {
        self.questionnaire_data = Some(questionnaire);
    }

    #[method]
    fn update_form_notes(&mut self, form_id: u64, notes: String) {
        match &mut self.questionnaire_data {
            Some(q) => {
                q.update_form_notes(form_id, notes);
            },
            None => {
                godot_error!("Empty Questionnaire");
            },
        }
    }

    #[method]
    fn update_form_status(&mut self, form_id: u64, status: GDQuestionStatus) {
        let status = status.0;

        match &mut self.questionnaire_data {
            Some(q) => {
                if let Err(e) = q.update_form_status(form_id, status) {
                    godot_error!("{}", e);
                }
            },
            None => {
                godot_error!("Empty Questionnaire");
            }
        }
    }

    #[method]
    fn generate_report(&self, file_path: String, font_style: (String, String)) {
        match &self.questionnaire_data {
            Some(qs) => { 
                qs.to_pdf(file_path, String::from("Testing Document"),font_style);
                self.draw_plot();
            },
            None => { 
                godot_error!("No questionnaire available");
            }
        }
    }

    #[method]
    fn all_form_fields(&self) -> Vec<(u64, String, String)> {
        let result = &self.questionnaire_data;
        match result {
            Some(questionnaire) => {

                let all_id_form = questionnaire.all_forms();

                let mut form_info = vec![];

                for (id, unit_form) in &all_id_form {
                    let question = questionnaire.get_question(unit_form.get_section_id(), unit_form.get_question_id()).unwrap();
                    let section = questionnaire.get_section(unit_form.get_section_id()).unwrap();

                    form_info.push((**id, section.get_title(), question.get_title()))
                }

                form_info

            },
            None => vec![],
        }
    }

    pub fn questionnaire_test() -> Questionnaire {
        let question = Question::new(Id::<Question>::new(0),String::from("question 1"), String::from("my question"));
        let mut section = Section::new(0, String::from("section 1"), String::from("my section"), HashMap::new());
        section.add_question(question);
        let mut job = Job::new(0, String::from("job 1"), String::from("my job"), HashMap::new());
        job.add_section(section);

        Questionnaire::new(job)
    }

    fn job_query_to_job_questionnaire(job_queries: Vec<JobSpecificationSection>) -> Result<HashMap<u64, Job>, &'static str> {
        let mut hash_jobs = HashMap::new();

        for job_spec in job_queries.iter() {
            // 1. check if job already exsits
            let job_id = job_spec.job_type_id;

            if hash_jobs.contains_key(&job_id) == false {
                // Create job
                let job_name = job_spec.job_name.clone();
                let mut job = Job::new(job_id, job_name, String::from(""), HashMap::new());
                hash_jobs.insert(job_id, job);
            }

            // 2. check if section exists
            let mut job = hash_jobs.get_mut(&job_id).unwrap();

            let op_section = &job_spec.section;
            match op_section {
                Some(s) => {
                    let section_id = Id::<Section>::new(s.id);


                    if job.has_section(&section_id) == false {
                        let section_name = s.section_name.clone();

                        let section = Section::new(section_id.primitive(), section_name, String::from(""), HashMap::new());
                        job.add_section(section);
                    }

                    // Add specification
                    let specification_id = job_spec.specification_id;
                    let specification_content = job_spec.specification_content.clone();

                    let spec = Question::new(Id::<Question>::new(specification_id), specification_content, String::from(""));

                    job.add_question(section_id,spec)?;
                },
                None    => { 
                    let specification_id = job_spec.specification_id;
                    let specification_content = job_spec.specification_content.clone();

                    let spec = Question::new(Id::<Question>::new(specification_id), specification_content, String::from(""));
                    // Add as an orphan
                    job.add_orphaned_question(spec)?;
                }
            }
        }

        Ok(hash_jobs)

    }
}
