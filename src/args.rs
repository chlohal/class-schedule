use std::{env::args_os, path::PathBuf};

use crate::{load_course_input::load_course_file, course_types::{CourseCredit}, load_classes_taken_input::load_classes_taken, course_library::CourseLibrary};



#[derive(Debug, Default)]
pub struct ProgramArgs {
    pub goals: Vec<Goal>,
    pub position: Vec<CourseCredit>,
    pub courses: CourseLibrary,
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
            arg.len()
        }
    };

    let (k, v) = arg.split_at(eq_position);

    let name_key = k.replace('-', "");

    let value = &v[1..];

    if name_key == "coursesfile" {
        match load_course_file(&PathBuf::from(value)) {
            Ok(courses) => parsed_args.courses.append(courses),
            Err(err) => eprintln!("Error adding courses: {:?}", err)
        }
    }

    if name_key == "classestakenfile" {
        load_classes_taken(&PathBuf::from(value), &parsed_args.courses);
    }
}
