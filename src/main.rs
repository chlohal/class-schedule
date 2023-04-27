#![feature(generators, generator_trait)]

use args::args_parsed;
use find_best_strategy::find_best_strategy;

mod load_classes_taken_input;
mod load_course_input;
mod args;
mod course_types;
mod helpers;
mod course_library;
mod find_best_strategy;
mod evaluate_strategy;
mod overlaps;
mod top_n;

pub const KEEP_BEST_N: usize = 10;
pub const NUM_COURSES_MAX: usize = 4;

fn main() {
    let mut args = args_parsed();
    
    if !args.extra.contains(&"keepunspecified".to_string()) {
        println!("filtering");
        args.courses.filter(|course| !course.schedule.is_empty());
    }

    println!("{:#?}", args.courses.iter().collect::<Vec<_>>());

    if args.extra.contains(&"count".to_string()) {
        println!("{}", args.courses.len());
    } else {
        for c in find_best_strategy(args, KEEP_BEST_N).best_n {
            println!("{}: {:?}", c.score, c.courses.iter().map(|x| x.course.course.to_string()).collect::<Vec<_>>().join(" "))
        }
    }
}
