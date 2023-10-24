use gdnative::api::{Resource};
use gdnative::prelude::*;

use questionnaire::data::{Id, Question, Section};
use questionnaire::job::Job;
use questionnaire::questionnaire::{UnitForm, QuestionStatus, Questionnaire};

use std::collections::HashMap;

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


#[derive(NativeClass)]
#[inherit(Resource)]
pub struct QuestionnaireData {
    pub questionnaire: Option<Questionnaire>,
}

#[methods]
impl QuestionnaireData {
    fn new(_owner: &Resource) -> Self {
        Self { questionnaire: None::<Questionnaire> }
    }

    pub fn set_job(&mut self, questionnaire: Questionnaire) {
        self.questionnaire = Some(questionnaire);
    }

    #[method]
    fn update_form_notes(&mut self, form_id: u64, notes: String) {
        match &mut self.questionnaire {
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

        match &mut self.questionnaire {
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
    fn all_form_fields(&self) -> Vec<(u64, String, String)> {
        match &self.questionnaire {
            Some(q) => {
                let all_id_form = q.all_forms();
                let mut form_info = vec![];

                for (id, unit_form) in &all_id_form {
                    let question = q.get_question(unit_form.get_section_id(), unit_form.get_question_id()).unwrap();
                    let section = q.get_section(unit_form.get_section_id()).unwrap();

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
}

