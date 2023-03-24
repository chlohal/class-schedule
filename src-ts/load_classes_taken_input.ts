import { readFileSync } from "node:fs";
import { CourseLibrary } from "./course_library.js";
import { Course, CourseAttribute, CourseCredit } from "./course_types.js";
import { parse_pipe_sep_list_key } from "./load_course_input.js";

export function load_classes_taken(
    path: string,
    course_library: CourseLibrary,
): CourseCredit[] {
    let f = readFileSync(path).toString();

    let name_arr = JSON.parse(f);

    let credits = name_arr
        .map((x: any) => typeof x === "string" 
        ? (course_library.getByName(x.trim()) || 
            (console.warn(`In order to ensure proper evaluation of your credits from ${x}, please manually specify its 'course' and 'attr_code'`) , []) )
        : ensureIsCourseCredit(x) 
        ).flat();

    for (const c of credits) c.taken_by_user = true;

    return credits;
}


function ensureIsCourseCredit(x: any): CourseCredit {
    if(typeof x !== "object") throw new Error(`Could not coerce value '${x}' to a course credit. Please specify 'course' and 'attr_code' attributes.`);

    if(typeof x.course !== "string") throw new Error(`Could not coerce value '${x}' to a course credit. Please ensure 'course' is a string.`);

    let attr_code = x.attr_code;

    if(typeof attr_code === "string") attr_code = parse_pipe_sep_list_key(x, "attr_code", x => x as CourseAttribute);

    return {
        course: x.course,
        attr_code: attr_code as CourseAttribute[]
    }
}