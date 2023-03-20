use args::args_parsed;



mod load_input;
mod args;
mod course_types;

fn main() {
    let args = args_parsed();
    println!("{:?}", args);
}
