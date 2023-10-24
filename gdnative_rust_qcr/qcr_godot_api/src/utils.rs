use sql_database::db::{JobSpecificationSection};
use sql_database::db;
use std::collections::HashMap;
use questionnaire::job::Job;
use questionnaire::data::{Question,Id, Section};


pub fn job_query_to_job_questionnaire(job_queries: Vec<JobSpecificationSection>) -> Result<HashMap<u64,Job>, &'static str> {
    // get all jobs first:
    let mut job_ids_name: Vec<(u64, String)> = Vec::new();
    for x in &job_queries {
        job_ids_name.push((x.job_type_id,x.job_name.clone()));
    }
                        
    // 2. Fill a hashmap with jobs.
    let mut job_hash_map = HashMap::new();

    for (id,name) in job_ids_name.iter() {
        job_hash_map.insert(*id, Job::new(*id, name.clone(), String::from(""), HashMap::new()));
    }

    for job_spec_sec in &job_queries {
        // 1. Check for section
        let op_job = job_hash_map.get_mut(&job_spec_sec.job_type_id);
        match op_job {
            Some(_) => {
                // Get job from hashmap
                let mut job = job_hash_map.get_mut(&job_spec_sec.job_type_id).unwrap();

                // Prepare section
                match &job_spec_sec.section {
                    Some(db_section) =>{
                        // Add section if not found
                        if job.has_section(&Id::<Section>::new(db_section.id)) == false {
                            let section = Section::new(db_section.id, db_section.section_name.clone(), String::from(""), HashMap::new());
                            job.add_section(section);
                        }

                        
                        let question = Question::new(Id::<Question>::new(job_spec_sec.specification_id), job_spec_sec.specification_content.clone(), String::from(""));
                        job.add_question(Id::<Section>::new(db_section.id), question);
                    },
                    None => {
                        let question = Question::new(Id::<Question>::new(job_spec_sec.specification_id), job_spec_sec.specification_content.clone(), String::from(""));
                        job.add_orphaned_question(question);
                    }
                }
            },
            None => { return Err("No job id found")},
        }
    }

    Ok(job_hash_map)


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_translation() {
        let q0 = Question::new(Id::<Question>::new(0), String::from("Title Page"), String::from(""));
        let q1 = Question::new(Id::<Question>::new(1), String::from("Subtitle"), String::from(""));

        let mut section = Section::new(0,String::from("Cover Page"),String::from(""), HashMap::new());
        let mut section_two = Section::new(1,String::from("Well Data"),String::from(""), HashMap::new());
        let q2 = Question::new(Id::<Question>::new(2), String::from("Well Percentage"), String::from(""));
        let q3 = Question::new(Id::<Question>::new(3), String::from("Location"), String::from(""));

        section.add_question(q0);
        section.add_question(q1);

        section_two.add_question(q2);
        section_two.add_question(q3);

        let mut expected_job = Job::new(0, String::from("Cement"), String::from(""), HashMap::new());
        expected_job.add_section(section);
        expected_job.add_section(section_two);

        let db_section = db::Section { id: 0, section_name: String::from("Cover Page") };
        let db_section_two = db::Section { id: 1, section_name: String::from("Well Data") };


        /* JobSpecificationSection */
        let js1 = JobSpecificationSection::new(0,String::from("Cement"),Some(db_section.clone()),0,String::from("Title Page"));
        let js2 = JobSpecificationSection::new(0,String::from("Cement"),Some(db_section.clone()),1,String::from("Subtitle"));

        let js3 = JobSpecificationSection::new(0,String::from("Cement"),Some(db_section_two.clone()),2,String::from("Well Percentage"));
        let js4 = JobSpecificationSection::new(0,String::from("Cement"),Some(db_section_two.clone()),3,String::from("Location"));

        let vs = vec![js1,js2,js3,js4];

        let op_hash_job = job_query_to_job_questionnaire(vs);

        match op_hash_job {
            Ok(jobs) => assert_eq!(expected_job, *jobs.get(&0).unwrap()),
            Err(e)   => { 
                println!("{}",e);
                assert!(false) },
        }

    }
}
