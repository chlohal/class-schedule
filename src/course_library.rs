use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::course_types::{Course, CourseSection};

#[derive(Debug, Default)]
pub struct CourseLibrary {
    sections: Vec<Arc<CourseSection>>,
    course_name_index: HashMap<String, Weak<Course>>,
    section_crn_index: HashMap<u32, Weak<CourseSection>>
}

impl CourseLibrary {
    fn add_course(&mut self, course: Weak<Course>) {
        if let Some(course) = course.upgrade() {
            if !self
                .course_name_index
                .contains_key(Arc::as_ref(&course.course))
            {
                self.course_name_index
                    .insert(course.course.to_string(), Arc::downgrade(&course));
            }
        }
    }
    pub fn add_section(&mut self, section: Arc<CourseSection>) {
        self.add_course(Arc::downgrade(&section.course));

        self.section_crn_index.insert(section.crn, Arc::downgrade(&section));

        self.sections.push(section);
    }
    pub fn get_by_name(&self, name: &String) -> Option<Weak<Course>> {
        self.course_name_index.get(name).cloned()
    }
    pub fn get_by_crn(&self, crn: &u32) -> Option<Weak<CourseSection>> {
        self.section_crn_index.get(&crn).cloned()
    }

    pub fn iter(&self) -> std::slice::Iter<Arc<CourseSection>> {
        return self.sections.iter();
    }

    pub fn filter<F: FnMut(&Arc<CourseSection>) -> bool>(&mut self, f: F) {
        self.sections.retain(f);
    }

    pub fn len(&self) -> usize {
        self.sections.len()
    }
}
