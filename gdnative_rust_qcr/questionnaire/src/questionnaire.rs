use crate::data::{Id,Question, Section};
use crate::job::Job;
use std::collections::HashMap;
use std::fmt::Display;
use std::{error::Error,io};

use serde::Serialize;

use crate::CSVWrite;

use genpdf::elements;

pub trait PDFable {
    fn to_pdf(&self, file_path: String, title: String, font_style: (String, String));
}

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum QuestionStatus {
    OK,
    NA,
    NO,
}

impl Display for QuestionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            QuestionStatus::OK => "OK".to_string(),
            QuestionStatus::NA => "N/A".to_string(),
            QuestionStatus::NO => "NO".to_string(),
        };

        write!(f,"{}",s)
    }
}

#[derive(Debug,PartialEq)]
pub struct UnitForm {
    u_id: u64,

    q_id: Id<Question>,
    s_id: Id<Section>,

    status: QuestionStatus,
    notes: String,
}

impl UnitForm {
    pub fn new(id: u64, q_id: Id<Question>, s_id: Id<Section>, status: QuestionStatus, notes: String) -> Self {
        UnitForm {
            u_id: id,
            q_id,
            s_id,
            status,
            notes,
        }
    }

    pub fn get_question_id(&self) -> &Id<Question> {
        &self.q_id
    }

    pub fn get_section_id(&self) -> &Id<Section> {
        &self.s_id
    }

    pub fn update_status(&mut self, status: QuestionStatus) {
        self.status = status;
    }

    pub fn update_notes(&mut self, note: String) {
        self.notes = note;
    }

    pub fn get_status(&self) -> QuestionStatus {
        self.status
    }
}

// Used for CSV writing of questionnaire
#[derive(Debug,Serialize)]
struct UnitFormRecord {
    section_name: String,
    specification_content: String,
    notes: String,
    status: String
}

#[derive(Debug)]
pub struct Questionnaire {
    job: Job,
    pub forms: HashMap<u64,UnitForm>,
}


impl Display for Questionnaire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (key, val) in self.forms.iter() {
            let section_string = match self.job.get_section(&val.s_id) {
                Ok(s) => s.get_title(),
                Err(_) => "".to_string()
            };

            let question_string = match self.job.get_question(&val.s_id, &val.q_id) {
                Ok(q) => q.get_title(),
                Err(_) => "".to_string()
            };
            
            let row = format!("{}, {}, {}, {}, {}\n",key, section_string, question_string, val.status, val.notes);
            s.push_str(&row);
        }

        write!(f,"{}",s)
    }
}

// TODO: Creation of unit form is not complete.
impl Questionnaire {
    pub fn new(job: Job) -> Self {
        let mut unit_form_id = 0;
        let mut hash_map: HashMap<u64, UnitForm> = HashMap::new();

        for (section_key, section) in job.sections.iter() {
            let question_hash = section.all_questions();

            for (question_key, _) in question_hash.iter() {
                let uf = UnitForm::new(unit_form_id, Id::<Question>::new(*question_key), Id::<Section>::new(*section_key), QuestionStatus::NA, String::from(""));

                hash_map.insert(uf.u_id, uf);
                unit_form_id += 1;
            }

        }

        Questionnaire { job, forms: hash_map } 
         
    }

    pub fn get_question(&self, s_id: &Id<Section>, q_id: &Id<Question>) -> Result<&Question, &'static str> {
            self.job.get_question(s_id, q_id) 
    }

    pub fn get_section(&self, s_id: &Id<Section>) -> Result<&Section, &'static str> {
            self.job.get_section(s_id) 
    }

    pub fn update_form_notes(&mut self, id: u64, note: String) -> Result<(), &'static str> {
        match self.forms.get_mut(&id) {
            Some(form) => { 
                form.update_notes(note);
                Ok(())
            },
            None => Err("Unable to update notes")
        }
    }

    pub fn update_form_status(&mut self, id: u64, status: QuestionStatus) -> Result<(), &'static str> {
        match self.forms.get_mut(&id) {
            Some(form) => {
                form.update_status(status);
                Ok(())
            },
            None => Err("Unable to update status")
        }
    }

    pub fn get_form(&mut self, id: u64) -> Result<&mut UnitForm, &'static str> {
        match self.forms.get_mut(&id) {
            Some(form) => Ok(form),
            None => Err("No form")
        }
    }

    pub fn all_forms(&self) -> Vec<(&u64, &UnitForm)> {
        let mut vs = Vec::new();
        for (k,v) in self.forms.iter() {
            vs.push((k,v));
        }

        vs.sort_by_key(|e| e.0);
        vs
    }

    fn to_unit_records(&self) -> Vec<UnitFormRecord> {
        let forms = self.all_forms();

        // go over forms to create the records froms verions, using jobs
        let mut records = Vec::new();


        for (_form_id, form) in forms.iter() {
            // desired information for the record.
            let section_id = form.get_section_id();
            let specification_id = form.get_question_id();

            let section = self.job.get_section(&section_id).unwrap();
            let specification = self.job.get_question(&section_id, &specification_id).unwrap();

            let section_name = section.get_title();
            let specification_content = specification.get_title();

            let notes = form.notes.clone();
            let status = form.get_status().to_string();

            // Build record
            let new_record = UnitFormRecord {
                section_name,
                specification_content,
                notes,
                status,
            };

            records.push(new_record);
        }

        records
    }

}

impl PDFable for Questionnaire {
    fn to_pdf(&self, file_path: String, title: String, font_style: (String,String)) {
                
        let font_family = genpdf::fonts::from_files(&font_style.0,&font_style.1, None)
            .expect("Failed to load font family");
        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);
        // Change the default settings
        doc.set_title(title);

        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        // Landscape A4 paper
        doc.set_paper_size((297,210));


        // Prepare table
        let mut table = elements::TableLayout::new(vec![1,1,1,1]);
        table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

        let unit_forms = self.all_forms();

        // Build rows
        for (_form_id, unit_form) in unit_forms.iter() {
            // Acquire information
            let section_id = unit_form.get_section_id();
            let specification_id = unit_form.get_question_id();

            // Information needed for report.
            let section = self.job.get_section(section_id).unwrap();
            let specification = self.job.get_question(section_id, specification_id).unwrap();
            let notes = unit_form.notes.clone();
            let status = unit_form.status;

            let mut row = table.row();

            row.push_element(elements::Paragraph::new(section.get_title()));
            row.push_element(elements::Paragraph::new(specification.get_title()));
            row.push_element(elements::Paragraph::new(notes));
            row.push_element(elements::Paragraph::new(status.to_string()));

            row.push().expect("invalid table row");

        }

        doc.push(table);


        doc.render_to_file(file_path).expect("Failed to write PDF file");
    }


}

impl CSVWrite for Questionnaire {
    fn write_csv(&self, file_path: String) -> Result<(), Box<dyn Error>> {
        let unit_records = self.to_unit_records();

        let mut wtr = csv::Writer::from_path(file_path)?;

        for record in unit_records.iter() {
            wtr.serialize(record)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

mod test {
    use super::*;

    fn unitform_test() -> UnitForm {
        UnitForm::new(0,Id::<Question>::new(0), Id::<Section>::new(0), QuestionStatus::OK, String::new())
    }

    fn questionnaire_test() -> Questionnaire {
        let question = Question::new(Id::<Question>::new(0),String::from("question 1"), String::from("my question"));
        let mut section = Section::new(0, String::from("section 1"), String::from("my section"), HashMap::new());
        section.add_question(question);
        let mut job = Job::new(0, String::from("job 1"), String::from("my job"), HashMap::new());
        job.add_section(section);

        Questionnaire::new(job)
    }

    #[test]
    fn new_unitform() {
        let uf = unitform_test();

        assert_eq!(0, uf.u_id);
        assert_eq!(Id::<Question>::new(0).primitive(), uf.q_id.primitive());
        assert_eq!(Id::<Section>::new(0).primitive(), uf.s_id.primitive());

        assert_eq!(QuestionStatus::OK, uf.status);
        assert_eq!(String::new(), uf.notes);
    }

    #[test]
    fn update_status() {
        let mut uf = unitform_test();

        uf.update_status(QuestionStatus::NO);

        assert_eq!(QuestionStatus::NO, uf.status);
    }

    #[test]
    fn update_notes() {
        let mut uf = unitform_test();

        uf.update_notes(String::from("This is an update"));

        assert_eq!(String::from("This is an update"), uf.notes);
    }

    #[test]
    fn questionnaire_create() {

        let mut qs = questionnaire_test();

        if let Err(_) = qs.get_form(0) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn questionnaire_update_form_notes() {
        let mut qs = questionnaire_test();

        if let Err(_) = qs.update_form_notes(0,String::from("testing this unit")) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn questionnaire_update_form_status() {
        let mut qs = questionnaire_test();

        if let Err(_) = qs.update_form_status(0,QuestionStatus::OK) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn questionnaire_display() {
        let mut qs = questionnaire_test();

        let q_string = format!("{}",qs);

        assert_eq!("0, section 1, question 1, N/A, \n",q_string);

    }

    #[test]
    fn questionnaire_display_note_update() {
        let mut qs = questionnaire_test();

        let q_string = format!("{}",qs);

        assert_eq!("0, section 1, question 1, N/A, \n",q_string);
        qs.update_form_notes(0,String::from("my update of notes"));

        let q_string = format!("{}",qs);

        assert_eq!("0, section 1, question 1, N/A, my update of notes\n",q_string);

    }

    fn questionnaire_display_status_update() {
        let mut qs = questionnaire_test();

        let q_string = format!("{}",qs);

        assert_eq!("0, section 1, question 1, N/A, \n",q_string);
        qs.update_form_status(0,QuestionStatus::OK);

        let q_string = format!("{}",qs);

        assert_eq!("0, section 1, question 1, OK, \n",q_string);

    }
}

