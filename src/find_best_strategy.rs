use std::{
    char::MAX,
    collections::{HashSet, VecDeque},
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    args::ProgramArgs,
    course_types::{Course, CourseSection},
    evaluate_strategy::evaluate_strategy,
    overlaps::{classes_overlap, NonOverlappingSections},
    top_n::TopN,
};

const MAX_COURSES: usize = 4;

#[derive(Debug, Default, Clone)]
pub struct Strategy {
    pub courses: Vec<Arc<CourseSection>>,
    pub score: f64,
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
    let possible_combination_cells = NonOverlappingSections::new(&args.courses);

    println!(
        "{}x{}",
        possible_combination_cells.0.len(),
        possible_combination_cells.max_possible()
    );

    let cell_count = possible_combination_cells.0.len();

    let mut top_strats: TopN<Strategy> = TopN::new(keep_num);

    let mut done = 0;
    for possibilities in possible_combination_cells.0.into_iter() {
        for best_strat in choose_possible_strategies(&possibilities, keep_num) {
            top_strats.insert(best_strat);
        }
        println!("{}/{} ({})", done, cell_count, possibilities.len());
        done += 1;
    }

    ProgramResults { best_n: top_strats }
}

fn choose_possible_strategies(
    universe: &HashSet<Arc<CourseSection>>,
    keep_num: usize,
) -> TopN<Strategy> {
    let mut top_strats: TopN<Strategy> = TopN::new(keep_num);

    for courses in universe.iter().permutations(4) {
        let strat = Strategy::from(courses.into_iter().map(|x| x.clone()).collect::<Vec<_>>());

        top_strats.insert(strat);
    }

    top_strats
}
