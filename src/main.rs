use args::args_parsed;


mod load_classes_taken_input;
mod load_course_input;
mod args;
mod course_types;
mod helpers;
mod course_library;

fn main() {
    let args = args_parsed();
    println!("{:?}", args);
}
