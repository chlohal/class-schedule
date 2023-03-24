import { CourseLibrary } from "./course_library.js";
import { Course, CourseCredit, Goal } from "./course_types.js";
import { load_classes_taken } from "./load_classes_taken_input.js";
import load_course_file from "./load_course_input.js";
import { load_goals } from "./load_goals_input.js";

export type ProgramArgs = {
    goals: Goal[],
    taken: CourseCredit[],
    courses: CourseLibrary,
    extra: Partial<Record<string, string>>
}

export function args_parsed(): ProgramArgs {
    let parsed_args: ProgramArgs = { goals: [], taken: [], courses: new CourseLibrary(), extra: {} };

    for (const arg of process.argv.slice(2)) handle_argument(parsed_args, arg);

    return parsed_args;
}

function handle_argument(parsed_args: ProgramArgs, arg: string) {

    let [k, value] = arg.split("=", 2);

    let name_key = k.replace(/-/g, "");

    parsed_args.extra[name_key] = value;

    if (name_key == "coursesfile") {
        parsed_args.courses.addCourse(...load_course_file(value));
    } 
    
    if(name_key == "coursestakenfile") {
        parsed_args.taken.push(...load_classes_taken(value, parsed_args.courses));
    }

    if (name_key == "goalsfile") {
        parsed_args.goals.push(... load_goals(value));
    }
}
