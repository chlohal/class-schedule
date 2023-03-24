use std::{collections::HashMap, ops::Index};

use crate::course_types::Course;

#[derive(Debug, Default)]
pub struct CourseLibrary {
    courses: Vec<Course>,
    course_name_index: HashMap<String, Vec<usize>>,
}

impl CourseLibrary {
    pub fn append(&mut self, courses: Vec<Course>) {
        for course in courses {
            self.courses.push(course);
            match self.course_name_index.get_mut(&course.course) {
                Some(course_sections) => {
                    course_sections.push(self.courses.len());
                }
                None => {
                    let mut course_sections = Vec::new();
                    course_sections.push(self.courses.len());
                    self.course_name_index
                        .insert(course.course, course_sections);
                }
            }
        }
    }
    pub fn schedule()
}

impl Index<String> for CourseLibrary {
    type Output = Option<Vec<Course>>;

    fn index(&self, index: String) -> &Self::Output {
        &self.course_name_index
            .get(&index)
            .map(|is| is.iter().map(|i| self.courses[*i]).collect()
        )
    }
}
