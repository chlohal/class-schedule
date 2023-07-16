use std::{
    collections::HashSet,
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    args::ProgramArgs,
    course_types::{CourseSection},
    evaluate_strategy::evaluate_strategy,
    overlaps::{NonOverlappingSections, self, classes_overlap, class_overlaps_with_classes},
    top_n::TopN, NUM_COURSES_MAX, course_library::CourseLibrary,
};

#[derive(Debug, Default, Clone)]
pub struct Strategy {
    pub courses: Vec<Arc<CourseSection>>,
    pub score: f64,
}
impl Strategy {
    pub fn from_crns(base: &Vec<u32>, universe: &CourseLibrary) -> Strategy {
        base
            .iter()
            .map(|crn| universe.get_by_crn(crn)
                .expect("couldn't find base CRN").upgrade()
                .expect("couldn't upgrade weak pointer for base CRN"))
            .collect::<Vec<_>>()
            .into() 
    }

    pub fn len(&self) -> usize {
        self.courses.len()
    }

    fn insert(&mut self, section: Arc<CourseSection>) {
        self.courses.push(section);
        self.score = evaluate_strategy(&self.courses);
    }
}

impl From<Vec<Arc<CourseSection>>> for Strategy {
    fn from(value: Vec<Arc<CourseSection>>) -> Self {
        Strategy {
            score: evaluate_strategy(&value),
            courses: value,
        }
    }
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.courses == other.courses && self.score == other.score
    }
}

impl PartialOrd for Strategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Eq for Strategy {}

impl Ord for Strategy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.score.total_cmp(&other.score);
    }
}

#[derive(Default, Debug)]
pub struct ProgramResults {
    pub best_n: TopN<Strategy>,
}

pub fn find_best_strategy(args: ProgramArgs, keep_num: usize) -> ProgramResults {

    let mut top_strats: TopN<Strategy> = TopN::new(keep_num);

    let base: Strategy = Strategy::from_crns(&args.base, &args.courses);

    for possibilities in args.courses.iter().permutations(NUM_COURSES_MAX - base.len()) {
        let mut new = base.clone();
        for p in possibilities {
            if p.cap == p.enr { continue; }
            
            if !class_overlaps_with_classes(p, &new.courses) {
                new.insert(p.clone());
            }
        }

        if new.len() == NUM_COURSES_MAX {
            top_strats.insert(new);
        }
    }

    ProgramResults { best_n: top_strats }
}

fn choose_possible_strategies(
    universe: &HashSet<Arc<CourseSection>>,
    keep_num: usize,
) -> TopN<Strategy> {
    let mut top_strats: TopN<Strategy> = TopN::new(keep_num);

    for courses in universe.iter().permutations(NUM_COURSES_MAX) {
        let strat = Strategy::from(courses.into_iter().map(|x| x.clone()).collect::<Vec<_>>());

        top_strats.insert(strat);
    }

    top_strats
}
