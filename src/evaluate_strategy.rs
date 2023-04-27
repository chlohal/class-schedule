use std::{collections::HashSet, sync::Arc};

use crate::course_types::{CourseAttribute, CourseSection};

pub fn evaluate_strategy(strat: &Vec<Arc<CourseSection>>) -> f64 {
    let pls_requirements = strat
        .iter()
        .fold(HashSet::<CourseAttribute>::new(), |mut acc, item| {
            acc.drain()
                .chain(item.course.attr_code.iter().cloned())
                .collect()
        }).len();

    return pls_requirements as f64;
}
