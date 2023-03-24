use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

use serde_json::Value;

use crate::{course_library::CourseLibrary, course_types::CourseCredit, helpers::err};

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
        .map(|course_name| -> Result<String, Error> {
            Ok(
                course_name
                    .as_str()
                    .ok_or(err("Non-string item in classes-taken array"))?
                    .to_string(),
            )
        })
        .map(|course_name_str| Ok(course_library[course_name_str?]))
        .collect::<Result<Vec<CourseCredit>, Error>>()?
}
