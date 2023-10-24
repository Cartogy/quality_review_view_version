use gdnative::prelude::*;
use gdnative::api::{Resource, Control};
use questionnaire::data::{Question, Id};

//mod questionnaire_display;
mod qcreport;
mod questionnaire_data;
mod utils;
mod database_api;
mod job_database_api;

use questionnaire_data::QuestionnaireData;
use qcreport::QCReport;
use database_api::DatabaseAPI;
use job_database_api::JobDatabaseAPI;

//use questionnaire_display::QuestionnaireDisplay;

/*
struct GDQuestion(Question);

impl ToVariant for GDQuestion {
    fn to_variant(&self) -> Variant {
        let id = self.0.get_id().primitive();
        let text = self.0.get_title();

        QuestionVariant {id, text, description: String::from("testing") }.to_variant()
    }
}
*/

//1. Create QuestionResource
//2. Readonly resource
//3. Wrap Question to GDQuestion
//4. implement to and fromVAriant for GDQuestion using QuestionResource
//


/*
#[derive(NativeClass, ToVariant, FromVariant)]
#[inherit(Resource)]
struct QuestionResource {
    question: Option<QuestionVariant>
}

#[methods]
impl QuestionResource {
    fn new(_owner: &Resource) -> Self {
        QuestionResource { question: None }
    }

    #[method]
    fn from_dictionary(&mut self, fields: Dictionary) {
        let id = fields.get("id").unwrap().to_string().parse::<u64>().unwrap();
        let text = fields.get("text").unwrap().clone().to_string();
        let description = fields.get("description").unwrap().clone().to_string();

        self.question = Some(QuestionVariant {id, text, description });
    }

    #[method]
    fn get_text(&self) -> String {
        match &self.question {
            Some(q) => q.text.clone(),
            _       => "".to_string()
        }
    }
}

// Used to have questiom switch from and to VAriant.
/* methods not nedded for QV, because it serves as an intermediary structure **/
#[derive(NativeClass, ToVariant, FromVariant)]
#[no_constructor]
#[inherit(Resource)]
struct QuestionVariant {
    id: u64,
    text: String,
    description: String,
}


#[derive(NativeClass)]
#[inherit(Resource)]
struct QuestionFactory {}

#[methods]
impl QuestionFactory {
    fn new(_owner: &Resource) -> Self {
        Self {}
    }

    #[method]
    fn build_from_scratch(id: u64, text: String, description: String) -> GDQuestion {
        let q = Question::new(Id::<Question>::new(id), text, description);
        GDQuestion(q)
    }

    fn build(question: Question) -> GDQuestion {
        GDQuestion(question)
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
struct Questionnaire; 

#[methods]
impl Questionnaire {
    fn new(_owner: &Control) -> Self {
        Questionnaire
    }

    #[method]
    fn _ready(&self) {
        /*
        let instance = QuestionDisplay::new_instance();
        instance.map_mut(|q: &mut QuestionDisplay, _base: TRef<Control, Unique>| {
            q.set_question(Question::new(Id::<Question>::new(0), String::from("Test Question"), String::from("Test Description")));
        });
        */


    }
}

*/
/*
#[derive(NativeClass)]
#[inherit(Control)]
struct QuestionDisplay {
    q: Option<Question>,
}

#[methods]
impl QuestionDisplay {
    fn new(_owner: &Control) -> Self {
        QuestionDisplay {
            q: Some(Question::new(Id::<Question>::new(0), String::from("Test Question"), String::from("Test Description")))
        }
    }

    #[method]
    fn get_text(&self) -> String {
        match &self.q {
            Some(q) => q.get_title().clone(),
            None    => "".to_string(),
        }
    }

    fn set_question(&self, q: Question) {
        self.q = Some(q)
    }
}
*/

fn init(handle: InitHandle) {
    //handle.add_class::<QuestionVariant>();
    //handle.add_class::<QuestionFactory>();
    //handle.add_class::<QuestionResource>();
    handle.add_class::<QuestionnaireData>();
    handle.add_class::<QCReport>();
    handle.add_class::<DatabaseAPI>();
    handle.add_class::<JobDatabaseAPI>();
    //handle.add_class::<QuestionDisplay>();
}

godot_init!(init);
