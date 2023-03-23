import { ClassPeriod, ClassTime, Course, CourseAttribute, CourseDay, Department, TimeRange } from "./course_types.js"
import { readFileSync } from "node:fs";


export default function load_course_file(path: string): Course[] {
    let f = readFileSync(path).toString();

    let data_arr = JSON.parse(f).data;

    return data_arr.map(data_row_to_course);
}

function get_stringy_number_key(data: any, key: string): number {
    let num = +data[key];
    if (isNaN(num)) throw new Error(`Key ${key} is not a number!`);

    return num;
}

export function parse_pipe_sep_list_key<T>(
    data: any,
    key: string,
    parser: (_: string) => T
): T[] {
    let content = data[key];
    if (!content) return [];

    let pipes = content.split("|");

    return pipes.map(parser);
}

function get_yn_bool_key(data: any, key: string): boolean {
    if (data[key] === "Y") return true;
    else if (data[key] === "N") return false;
    else throw new Error(`Key ${key} is not Y or N; instead, it's ${data[key]}`);
}

function data_row_to_course(data: any): Course {
    return {
        campus_code: get_stringy_number_key(data, "campus_code"),
        term_code: data.term_code,
        ptrm_code: data.ptrm_code,
        crn: get_stringy_number_key(data, "crn"),
        sub_group_list: parse_pipe_sep_list_key(data, "sub_group_list", x => x as Department),
        course: data.course,
        seq_num: get_stringy_number_key(data, "seq_num"),
        title: data.title,
        long_title: data.long_title,
        attr_code: parse_pipe_sep_list_key(data, "attr_code", x => x as CourseAttribute),
        units: data.units,
        cap: data.cap,
        enr: data.enr,
        permission_only: get_yn_bool_key(data, "permission_only"),
        instructor: data.instructor,
        schedule: dayTimeToSchedule(
            parse_pipe_sep_list_key(data, "days", x => x == "-" ? undefined : x.split("") as CourseDay[]),
            parse_pipe_sep_list_key(data, "times", parseTimeRange)
        ),
        location: parse_pipe_sep_list_key(data, "building_abbrev", stripHTML),
        note: data.ssrtext_text,
    };
}

function dayTimeToSchedule(days: (CourseDay[] | undefined)[], times: (TimeRange | undefined)[]): ClassPeriod[] {
    if(days.length != times.length) throw new Error("Could not turn days/times to schedule");
    const periods: ClassPeriod[] = []

    for(let i = days.length; i >= 0; i--) {
        const day = days[i], time = times[i];
        if(day == undefined && time == undefined) continue;

        if(day == undefined || time == undefined) throw new Error("Only one of 'day' and 'time' is undefined: could not combine");

        periods.push({
            days: day,
            time: time
        })
    }

    return periods;
}

function stripHTML(source: string): string {
    let result = "";

    let depth = 0, quoted = false;

    for (const char of source) {
        if (char == '"') quoted = !quoted;
        if (!quoted && char == '<') depth++;
        else if (!quoted && char == '>') depth--;
        else if (!quoted && depth == 0) result += char;
    }

    return result;
}

function parseTimeRange(range: string): TimeRange | undefined {
    if (range == "-") return undefined;

    const [start, end] = range.split("-").map(parseTime);

    return {
        end, start
    }
}

function parseTime(time: string): ClassTime {
    const trimmed = time.trim();

    const [hourStr, minStr] = trimmed.split(":");

    const hour = +hourStr + (trimmed.endsWith("PM") ? 12 : 0);

    const minute = +minStr.substring(0, 2);

    if (isNaN(hour) || isNaN(minute)) throw new Error(`Bad time format ${time}`);

    return {
        hour,
        minute
    }
}