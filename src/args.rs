use std::{env::args_os, path::PathBuf, option::Iter};

use crate::{load_course_input::load_course_file, course_types::{CourseCredit}, load_classes_taken_input::load_classes_taken, course_library::CourseLibrary};



#[derive(Debug, Default)]
pub struct ProgramArgs {
    pub goals: Vec<Goal>,
    pub taken: Vec<CourseCredit>,
    pub courses: CourseLibrary,
    pub extra: Vec<String>,
    pub base: Vec<u32>
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
            parsed_args.extra.push(arg.replace('-', ""));
            return;
        }
    };

    let (k, v) = arg.split_at(eq_position);

    let name_key = k.replace('-', "");

    let value = &v[1..];

    if name_key == "coursesfile" {
        match load_course_file(&PathBuf::from(value)) {
            Ok(courses) => parsed_args.courses = courses,
            Err(err) => eprintln!("Error adding courses: {:?}", err)
        }
    }

    if name_key == "classestakenfile" {
        match load_classes_taken(&PathBuf::from(value), &parsed_args.courses) {
            Ok(mut credits) => parsed_args.taken.append(&mut credits),
            Err(err) => eprintln!("Error adding courses taken: {:?}", err),
        }
    }

    if name_key == "base" {
        parsed_args.base = value.split(",").map(|x| x.parse().expect("couldn't parse a CRN in the base argument")).collect()
    }
}
