use std::{env::args_os, path::PathBuf};

use crate::{load_input::load_course_file, course_types::Course};



#[derive(Debug, Default)]
pub struct ProgramArgs {
    pub goals: Vec<Goal>,
    pub courses: Vec<Course>,
}

#[derive(Debug)]
pub struct Goal {
    pub field: String,
    pub value: String,
    pub important: usize,
}

pub fn args_parsed() -> ProgramArgs {
    let mut parsed_args = ProgramArgs::default();
    let mut args = args_os();

    //ignore first element
    args.next();

    for os_arg in args {
        match os_arg.into_string() {
            Ok(arg) => handle_argument(&mut parsed_args, arg),
            Err(os_arg) => eprintln!("Argument {:?} is not valid Unicode", os_arg),
        }
    }

    parsed_args
}

fn handle_argument(parsed_args: &mut ProgramArgs, arg: String) {
    let eq_position = match arg.find("=") {
        Some(n) => n,
        None => {
            eprintln!("Bad argument format: you must use `term=value`.");
            return;
        }
    };

    let (name, value) = arg.split_at(eq_position);

    let name_key = name.replace('-', "");

    if name_key == "course" {
        match load_course_file(&PathBuf::from(value[1..].to_string())) {
            Ok(mut courses) => parsed_args.courses.append(&mut courses),
            Err(err) => eprintln!("Error adding courses: {:?}", err)
        }
    }
}
