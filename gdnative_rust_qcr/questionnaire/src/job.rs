use crate::data::{Id, Section, Question};

use std::collections::HashMap;


#[derive(Debug)]
pub struct Job {
    id: Id<Job>,
    title: String,
    description: String,
    pub sections: HashMap<u64,Section>,
    pub orphaned_specifications: HashMap<u64, Question>
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {

        let valid_section = {
            let mut valid = true;
            for (k,v) in self.sections.iter() {
                if v != other.sections.get(k).unwrap() {
                    valid = false;
                    break;
                }
            }
            valid
        };

        self.id == other.id && self.title == other.title && valid_section
    }
}

impl Job {
    pub fn new(p_id: u64, title: String, description: String, sections: HashMap<u64,Section>) -> Self {
        Job {
            id: Id::<Job>::new(p_id),
            title,
            description,
            sections,
            orphaned_specifications: HashMap::new()
        }
    }
    pub fn add_section(&mut self, section: Section) {
        if self.has_section(&section.get_id()) == false{
            self.sections.insert(section.get_id().primitive(), section);
        }
    }
    pub fn get_section(&self, id: &Id<Section>) -> Result<&Section, &'static str> {
        let id = id.primitive();

        match self.sections.get(&id) {
            Some(section) => Ok(&section),
            None      => Err("No section found")
        }

    }

    pub fn get_mut_section(&mut self, id: &Id<Section>) -> Result<&mut Section, &'static str> {
        match self.sections.get_mut(&id.primitive()) {
            Some(section) => Ok(section),
            None          => Err("No section found")
        }
    }

    pub fn remove_section(&mut self, id: Id<Section>) -> Result<(), &'static str> {
        match self.sections.remove(&id.primitive()) {
            Some(_) => Ok(()),
            None          => Err("No section found that could be removed")
        }
    }

    pub fn get_question(&self, s_id: &Id<Section>, q_id: &Id<Question>) -> Result<&Question, &'static str> {
        self.get_section(&s_id).unwrap().get_question(q_id)
    }

    pub fn add_question(&mut self, s_id: Id<Section>, q: Question) -> Result<(), &'static str> {
        let mut section: &mut Section = self.get_mut_section(&s_id).unwrap();
        section.add_question(q);

        Ok(())
    }

    pub fn get_orphaned_question(&self, q_id: &Id<Question>) -> Result<&Question, &'static str> {
        match self.orphaned_specifications.get(&q_id.primitive()) {
            Some(spec) => {
                Ok(spec)
            },
            None => {
                Err("No orphaned question")
            }
        }


    }

    pub fn add_orphaned_question(&mut self, question: Question) -> Result<(), &'static str> {
        self.orphaned_specifications.insert(question.get_id().primitive(), question);

        Ok(())
    }

    pub fn remove_orphaned_question(&mut self, q_id: &Id<Question>) -> Result<(), &'static str> {
        match self.orphaned_specifications.remove(&q_id.primitive()) {
            Some(_) => Ok(()),
            None    => Err("No orphaned question to remove")
        }
    }

    fn remove_question(&mut self, s_id: &Id<Section>, q_id: &Id<Question>) -> Result<() , &'static str>{
        if let Ok(s) = self.get_mut_section(s_id) {
            if let Ok(_) = s.get_question(q_id) {

                 s.remove_question(q_id)
            } else {
                Err("No question")
            }
        } else {
            Err("No section")
        }
    }

    pub fn has_section(&self, s_id: &Id<Section>) -> bool {
        self.sections.contains_key(&s_id.primitive())
    }
}

mod tests {
    use super::*;

    #[test]
    fn add_get_section() {
        let s = Section::new(0, String::from("Dummy section"), String::from("Testing section"), HashMap::new());

        let mut job = Job::new(0, String::from("Dummy job"), String::from("testing job"), HashMap::new());
        job.add_section(s);

        if let Err(_) = job.get_section(&Id::<Section>::new(0)) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn remove_section() {
        let s = Section::new(0, String::from("Dummy section"), String::from("Testing section"), HashMap::new());

        let mut job = Job::new(0, String::from("Dummy job"), String::from("testing job"), HashMap::new());
        job.add_section(s);

        if let Err(_) = job.remove_section(Id::<Section>::new(0)) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn add_question() {
        let s = Section::new(0, String::from("Dummy section"), String::from("Testing section"), HashMap::new());

        let mut job = Job::new(0, String::from("Dummy job"), String::from("testing job"), HashMap::new());
        job.add_section(s);

        let q = Question::new(Id::<Question>::new(0), String::from("Question dummy"), String::from("me"));

        if let Err(_) = job.add_question(Id::<Section>::new(0),q) {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn remove_question() {
        let s = Section::new(0, String::from("Dummy section"), String::from("Testing section"), HashMap::new());

        let mut job = Job::new(0, String::from("Dummy job"), String::from("testing job"), HashMap::new());
        job.add_section(s);

        let q = Question::new(Id::<Question>::new(0), String::from("Question dummy"), String::from("me"));

        if let Err(_) = job.add_question(Id::<Section>::new(0),q) {
            assert!(false);
        } else {
            if let Err(_) = job.remove_question(&Id::<Section>::new(0), &Id::<Question>::new(0)) {
                assert!(false)
            } else {
                assert!(true)
            }
        }
    }

    #[test]
    fn has_section() {
        let s = Section::new(0, String::from("Dummy section"), String::from("Testing section"), HashMap::new());

        let mut job = Job::new(0, String::from("Dummy job"), String::from("testing job"), HashMap::new());
        job.add_section(s);

        assert_eq!(true, job.has_section(&Id::<Section>::new(0)));
    }

}
