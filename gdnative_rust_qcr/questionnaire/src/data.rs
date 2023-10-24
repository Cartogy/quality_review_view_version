use std::marker::PhantomData;
use std::collections::HashMap;

// ID
#[derive(PartialEq, Eq, Debug, Clone, Copy,Hash)]
pub struct Id<T> {
    pub id: u64,
    marker: PhantomData<T>,
}


impl<T> Id<T> {
    pub fn new(primitive: u64) -> Self {
        Id { id: primitive, marker: PhantomData }
    }

    pub fn primitive(&self) -> u64 {
        self.id
    }
}

// -----------

#[derive(Debug, Clone )]
pub struct Question {
    id: Id<Question>,
    text: String,
    description: String,
}

impl Question {
    pub fn new(id: Id<Question>, text: String, description: String) -> Question {
        Question{
            id,
            text,
            description,
        }
    }

    pub fn get_id(&self) -> Id<Question> {
        self.id.clone()
    }

    pub fn get_title(&self) -> String {
        self.text.clone()
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.text == other.text
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub id: Id<Section>,
    title: String,
    description: String,
    questions: HashMap<u64,Question>
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        let bools: Vec<bool> = self.questions.iter().map(|(k,v)| other.questions.contains_key(k) && v == other.questions.get(k).unwrap()).collect();

        let valid_questions = {
            let mut valid = true;
            for b in bools {
                if !b {
                    valid = false;
                    break;
                }
            }

            valid
        };

        self.id == other.id && self.title == other.title && valid_questions
    }
}

impl Section {
    pub fn new(p_id: u64, title: String, description: String, questions: HashMap<u64,Question>) -> Self {
        Section {
            id: Id::<Section>::new(p_id),
            title,
            description, 
            questions,
        }

    }

    pub fn get_id(&self) -> Id<Section> {
        self.id.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn add_question(&mut self, question: Question) {
        if self.has_question(&question.get_id()) == false {
            self.questions.insert(question.get_id().primitive(), question);
        }
    }

    pub fn remove_question(&mut self, id: &Id<Question>) -> Result<(), &'static str>{
        match self.questions.remove(&id.primitive()) {
            Some(_) => Ok(()),
            None   => Err("No question to remove")
        }
    }

    pub fn get_question(&self, id: &Id<Question>) -> Result<&Question, &'static str> {
        match self.questions.get(&id.primitive()) {
            Some(q) => Ok(q),
            None    => Err("No question found")
        }

    }

    pub fn all_questions(&self) -> &HashMap<u64,Question> {
        &self.questions
    }

    pub fn has_question(&self, id: &Id<Question>) -> bool {
        self.questions.contains_key(&id.primitive())
    }
}



#[cfg(test)]
mod tests{
    use super::{Question,Id, Section};

    mod test_question {
        use super::{Question, Id, Section};

        #[test]
        fn basic_question() {
            let q1 = Question::new(Id::<Question>::new(0), String::from("Will this work?"), String::from("Testing Question"));
            let q_expect = Question {
                id: Id::<Question>::new(0),
                text: String::from("Will this work?"),
                description: String::from("Testing Question"),
            };

            assert_eq!(q_expect, q1);
        }
    }

    mod test_section {
        use super::*;

        #[test]
        fn valid_question() {
            let q1 = Question::new(Id::<Question>::new(0), String::from("Will this work?"), String::from("Testing Question"));
            let mut section = Section::new(0, String::from("section one"), String::from("Testing section"), std::collections::HashMap::new());
            section.add_question(q1);


            let actual_q = section.get_question(&Id::<Question>::new(0));
            if let Err(_) = actual_q {
                assert!(false)
            } else {
                assert!(true)
            }
        }

        #[test]
        fn invalid_question() {
            //let q1 = Question::new(Id::<Question>::new(0), String::from("Will this work?"), String::from("Testing Question"));
            let qs = std::collections::HashMap::new();
            let section = Section::new(0, String::from("section one"), String::from("Testing section"), qs);


            let actual_q = section.get_question(&Id::<Question>::new(0));
            if let Err(_) = actual_q {
                assert!(true)
            } else {
                assert!(false)
            }
        }

        #[test]
        fn add_question() {
            let q1 = Question::new(Id::<Question>::new(0), String::from("Will this work?"), String::from("Testing Question"));
            let mut section = Section::new(0, String::from("section one"), String::from("Testing section"), std::collections::HashMap::new());

            section.add_question(q1);

            let q = section.get_question(&Id::<Question>::new(0));
            if let Err(_) = q {
                assert!(false)
            } else {
                assert!(true)
            }
        }

        #[test]
        fn remove_question() {
            let q1 = Question::new(Id::<Question>::new(0), String::from("Will this work?"), String::from("Testing Question"));
            let mut section = Section::new(0, String::from("section one"), String::from("Testing section"), std::collections::HashMap::new());
            section.add_question(q1);

            if let Err(_) = section.remove_question(&Id::<Question>::new(0)) {
                println!("No question found");
                assert!(false)
            }

            if let Err(_) = section.get_question(&Id::<Question>::new(0)) {
                assert!(true)
            } else {
                assert!(false)
            }
        }
    }
}
