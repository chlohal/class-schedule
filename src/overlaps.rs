use std::{
    collections::{HashMap, HashSet},
    slice,
    sync::Arc,
};

use crate::{
    course_library::CourseLibrary,
    course_types::{ClassPeriod, CourseDay, CourseSection},
};

#[derive(Default)]
pub struct NonOverlappingSections(pub Vec<HashSet<Arc<CourseSection>>>);

impl NonOverlappingSections {
    pub fn new(lib: &CourseLibrary) -> NonOverlappingSections {
        let mut this = HashMap::<u32, HashSet<Arc<CourseSection>>>::new();


        for section in lib.iter() {
            let mut non_overlaps = HashSet::<_>::new();
            non_overlaps.insert(section.clone());
            for bection in lib.iter() {
                let mut addable = true;
                for cection in non_overlaps.iter() {
                    if classes_overlap(&cection, &bection) {
                        addable = false;
                        break;
                    }
                }

                if section != bection && section.course != bection.course && addable {
                    non_overlaps.insert(bection.clone());
                }
            }
            this.insert(section.crn, non_overlaps);
        }

        NonOverlappingSections(this.into_values().collect())
    }

    pub fn max_possible(&self) -> usize {
        self.0
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .map_or(0, |x| x.len())
    }
}

pub fn classes_overlap(a: &CourseSection, b: &CourseSection) -> bool {
    for (a_range_min, a_range_max) in minute_ranges(&a.schedule) {
        for (b_range_min, b_range_max) in minute_ranges(&b.schedule) {
            if (a_range_min <= b_range_max && a_range_max >= b_range_min)
                || (b_range_min <= a_range_max && b_range_max >= a_range_min)
                || (a_range_min >= b_range_min && a_range_max <= b_range_max)
                || (b_range_min >= a_range_min && b_range_max <= a_range_max)
            {
                return true;
            }
        }
    }
    return false;
}

pub fn class_overlaps_with_classes(aection: &Arc<CourseSection>, b: &Vec<Arc<CourseSection>>) -> bool {
    for bection in b {
        if classes_overlap(aection, bection) {
            return true;
        }
    }
    return false;
}

fn minute_ranges<'a>(periods: &'a Vec<ClassPeriod>) -> impl Iterator<Item = (u32, u32)> + 'a {
    periods.iter().flat_map(|period| {
        let t = &period.time;

        period.days.iter().map(|d| {
            (
                day_number(&d) + t.start.hour * 60 + t.start.minute,
                day_number(&d) + t.end.hour * 60 + t.end.minute,
            )
        })
    })
}

fn day_number(d: &CourseDay) -> u32 {
    24 * 60
        * match d {
            CourseDay::M => 0,
            CourseDay::T => 1,
            CourseDay::W => 2,
            CourseDay::R => 3,
            CourseDay::F => 4,
        }
}
