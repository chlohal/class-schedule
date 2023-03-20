use std::{
    fs::{File},
    io::{BufReader, Error, ErrorKind},
    path::PathBuf,
};

use serde_json::Value;

use crate::course_types::{Course, CourseAttribute, CourseDay, Department, TimeRange};

pub fn load_course_file(path: &PathBuf) -> Result<Vec<Course>, Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let json: Value = serde_json::from_reader(reader)?;

    let data_arr = json
        .as_object()
        .and_then(|x| x.get("data"))
        .and_then(|x| x.as_array());

    match data_arr {
        Some(data) => data_array_to_courses(data),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Bad JSON schema for courses: no 'data' key in top-level object",
        )),
    }
}

fn data_array_to_courses(data: &Vec<Value>) -> Result<Vec<Course>, Error> {
    let courses = data
        .iter()
        .map(|alleged_course| -> Option<Course> {
            let course = alleged_course.as_object()?;
            Some(Course {
                campus_code: (course.get("campus_code")?.as_str()?).parse().ok()?,
                term_code: (course.get("campus_code")?.as_str()?).parse().ok()?,
                ptrm_code: (course.get("campus_code")?.as_str()?).parse().ok()?,
                crn: (course.get("campus_code")?.as_str()?).parse().ok()?,
                sub_group_list: (course.get("sub_group_list")?.as_str()?)
                    .split("|")
                    .map(|d| d.parse().ok())
                    .collect::<Option<Vec<Department>>>()?,
                course: course.get("course")?.as_str()?.to_string(),
                seq_num: (course.get("seq_num")?.as_str()?).parse().ok()?,
                title: course.get("title")?.as_str()?.to_string(),
                long_title: course.get("long_title")?.as_str()?.to_string(),
                attr_code: course
                    .get("attr_code")?
                    .as_str()
                    .unwrap_or("")
                    .split("|")
                    .map(|d| d.parse().ok())
                    .collect::<Option<Vec<CourseAttribute>>>()?,
                units: (course.get("units")?.as_str()?).parse().ok()?,
                cap: course.get("cap")?.as_i64()? as u32,
                enr: course.get("enr")?.as_i64()? as u32,
                permission_only: (course.get("permission_only")?.as_str()?) == "Y",
                instructor: (course.get("instructor")?.as_str()?).to_string(),
                days: (course.get("days")?.as_str()?)
                    .chars()
                    .map(|d| d.try_into().ok())
                    .collect::<Option<Vec<CourseDay>>>()?,
                times: (course.get("times")?.as_str()?)
                    .split("|")
                    .map(|d| d.parse().ok())
                    .collect::<Option<Vec<TimeRange>>>()?,
                building_abbrev: course.get("building_abbrev")?.as_str()?.to_string(),
                note: course.get("ssrtext_text")?.as_str()?.to_string(),
            })
        })
        .collect::<Option<Vec<Course>>>();

    return courses.ok_or(Error::new(ErrorKind::InvalidData, "Invalid JSON Schema"));
}
