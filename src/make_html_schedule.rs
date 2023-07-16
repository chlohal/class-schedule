use std::hash::{Hash, Hasher};

use crate::{find_best_strategy::Strategy, course_types::{CourseDay, TimeOfDay, Course}};

const ESTIMATE_BYTE_PER_COURSE: usize = 700;

pub fn html_schedule(strat: &Strategy) -> String{
    let mut html = String::with_capacity(strat.len() * ESTIMATE_BYTE_PER_COURSE);

    html += r#"
        <div style="width:100vw;height:100vh;display:block;position:relative;background:#efffee; position:absolute" class="sched-root">
            <div style="position:absolute;width:20vw;height:100vh;"></div>
    "#;

    for section in strat.courses.iter() {
        for period in section.schedule.iter() {
            for day in period.days.iter() {
                html += format!(r#"
                    <div style="position:absolute; width:20vw;box-sizing:border-box;
                        padding-left:1em;position:absolute;left:{}%; top:{}vh; height: {}%; display: flex">
                    
                        <div style="display: block; position: relative; height: 100%; width: 100%; flex-grow: grow; background: {};">
                            <h3 style="margin: 0">{}</h3>
                            <span>{} - {}</span>
                        </div>
                    </div>
                "#,
                course_day_index(day) * 20,
                to_percentage_of_classtime(&period.time.start) * 100.0,
                (to_percentage_of_classtime(&period.time.end) - 
                    to_percentage_of_classtime(&period.time.start)) * 100.0,
                colour_course(&section.course),
                section.course.long_title,
                period.time,
                section.crn
            ).as_str();
            }
        }
    }

    html += "</div></div>";

    html

}

fn to_percentage_of_classtime(time: &TimeOfDay) -> f64 {
    let h = (time.hour - 8) as f64;
    let m = (time.minute as f64) / 60.0;

    let t = m + h;

    //classes run from 8am to 9pm in this universe
    return t / 13.0;
}

fn course_day_index(day: &CourseDay) -> usize {
    match day {
        CourseDay::M => 0,
        CourseDay::T => 1,
        CourseDay::W => 2,
        CourseDay::R => 3,
        CourseDay::F => 4,
    }
}

fn colour_course(course: &Course) -> &'static str {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    course.course.hash(&mut h);
    
    let r = h.finish() % 8;

    return match r {
        0 => "#f78764",
        1 => "#e7ff70",
        2 => "#9ebd6e",
        3 => "#98b9f2",
        4 => "#BCA8C7",
        5 => "#F46767",
        6 => "#efa8b8",
        7 => "#ecca41",
        _ => panic!("mod is inconsistent")
    }
}