use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
    str::FromStr,
};

use serde_json::{Map, Value};

use crate::{course_types::Course, helpers::err};

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

fn get_stringy_parsable_key<T: FromStr>(data: &Map<String, Value>, key: &str) -> Result<T, Error> {
    (data
        .get(key)
        .ok_or(err(&("No ".to_owned() + key + " property")))?
        .as_str()
        .ok_or(err(&(key.to_owned() + " property is not a string")))?)
    .parse()
    .map_err(|_| err(&(key.to_owned() + " does not represent a parsable type")))
}

fn parse_pipe_sep_list_key<T: FromStr>(
    data: &Map<String, Value>,
    key: &str,
) -> Result<Vec<T>, Error> {
    let string_content = match data
        .get(key)
        .ok_or(err(&("No ".to_owned() + key + " property")))?
        .as_str()
    {
        Some(s) => s,
        None => return Ok(Vec::new()),
    };

    string_content
        .split("|")
        .map(|d| {
            d.parse()
                .map_err(|_| err(&("Could not parse member of ".to_owned() + key + " list: " + d)))
        })
        .collect()
}

fn get_string_key(data: &Map<String, Value>, key: &str) -> Result<String, Error> {
    Ok(data
        .get(key)
        .ok_or(err(&("No ".to_owned() + key + " property")))?
        .as_str()
        .unwrap_or("")
        .to_string())
}

fn get_u32_key(data: &Map<String, Value>, key: &str) -> Result<u32, Error> {
    Ok(data
        .get(key)
        .ok_or(err(&("No ".to_owned() + key + " property")))?
        .as_u64()
        .ok_or(err(&(key.to_owned() + " property is not an integer")))? as u32)
}

fn get_yn_bool_key(data: &Map<String, Value>, key: &str) -> Result<bool, Error> {
    Ok(data
        .get(key)
        .ok_or(err(&("No ".to_owned() + key + " property")))?
        .as_str()
        .ok_or(err(&(key.to_owned().clone() + " property is not a string")))?
        == "Y")
}

fn data_array_to_courses(data: &Vec<Value>) -> Result<Vec<Course>, Error> {
    let courses = data
        .iter()
        .map(|alleged_course| -> Result<Course, Error> {
            let course = alleged_course
                .as_object()
                .ok_or(err("data field is not object!"))?;
            Ok(Course {
                campus_code: get_stringy_parsable_key(course, "campus_code")?,
                term_code: get_string_key(course, "term_code")?,
                ptrm_code: get_string_key(course, "ptrm_code")?,
                crn: get_stringy_parsable_key(course, "crn")?,
                sub_group_list: parse_pipe_sep_list_key(course, "sub_group_list")?,
                course: get_string_key(course, "course")?,
                seq_num: get_stringy_parsable_key(course, "seq_num")?,
                title: get_string_key(course, "title")?,
                long_title: get_string_key(course, "long_title")?,
                attr_code: parse_pipe_sep_list_key(course, "attr_code")?,
                units: get_string_key(course, "units")?,
                cap: get_u32_key(course, "cap")?,
                enr: get_u32_key(course, "enr")?,
                permission_only: get_yn_bool_key(course, "permission_only")?,
                instructor: get_string_key(course, "instructor")?,
                days: parse_pipe_sep_list_key(course, "days")?,
                times: parse_pipe_sep_list_key(course, "times")?,
                location: parse_pipe_sep_list_key(course, "building_abbrev")?,
                note: get_string_key(course, "ssrtext_text")?,
            })
        })
        .collect::<Result<Vec<Course>, Error>>();

    return courses;
}