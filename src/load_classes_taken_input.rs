use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

use serde_json::Value;
use serde_json::Value::Object;
use serde_json::Value::String;

use crate::{
    course_library::CourseLibrary,
    course_types::CourseCredit,
    helpers::err,
    load_course_input::{get_string_key, parse_pipe_sep_list_key},
};

pub fn load_classes_taken(
    path: &PathBuf,
    course_library: &CourseLibrary,
) -> Result<Vec<CourseCredit>, Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let json: Value = serde_json::from_reader(reader)?;

    json.as_array()
        .ok_or(err("Classes-taken file is not a JSON array"))?
        .iter()
        .map(|course_credit| -> Result<CourseCredit, Error> {
            match course_credit {
                String(name) => Ok(course_library.get_by_name(name)
                    .and_then(|x| x.upgrade())
                    .map(|x| CourseCredit {
                        attr_code: x.attr_code.clone(),
                        course: x.course.clone()
                }).ok_or(err(&("Class ".to_string()
                    + name
                    + " doesn't exist in the library; please explicitly specify its attributes.")))?),
                Object(map) => Ok(CourseCredit {
                    attr_code: parse_pipe_sep_list_key(map, "attr_code")?.into(),
                    course: get_string_key(map, "course")?.into(),
                }),
                _ => Err(err(
                    "Classes-taken file must only contain strings or objects",
                )),
            }
        })
        .collect::<Result<Vec<CourseCredit>, Error>>()
}
