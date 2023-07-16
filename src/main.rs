#![feature(generators, generator_trait)]

use args::args_parsed;
use find_best_strategy::find_best_strategy;

use crate::make_html_schedule::html_schedule;

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
mod make_html_schedule;

pub const KEEP_BEST_N: usize = 1000;
pub const NUM_COURSES_MAX: usize = 5;

fn main() {
    let mut args = args_parsed();
    
    if !args.extra.contains(&"keepunspecified".to_string()) {
        args.courses.filter(|course| !course.schedule.is_empty());
    }

    if args.extra.contains(&"count".to_string()) {
        println!("{}", args.courses.len());
    } else {
        println!(r#"<!DOCTYPE html><html>
        <head>
            <script>
                {}
            </script>
        </head><body style="margin: 0">"#, include_str!("../asset/html_vis_script.js"));
        for c in find_best_strategy(args, KEEP_BEST_N).best_n {
            println!("{}", html_schedule(&c));
        }
        println!("</body></html>");

    }
}
