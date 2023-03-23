export type CourseCredit = {
    course: string,
    attr_code: CourseAttribute[]
}

export type Strategy = {
    proposal: Set<Course>
}

export type GoalField = string;
export type GoalTarget = GoalTargetValue | { direction: "up" | "down" };
export type GoalTargetValue = Set<string>;

export type Goal = {
    field: GoalField,
    target: GoalTarget,
    important: number,
}

export function courseToString(c: Course) {
    return `${c.course} (${
        c.schedule.map(x => `${x.days.join(",")}`).join("; ")
    })`
}

export type Course = {
    campus_code: number,
    term_code: string,
    ptrm_code: string,
    crn: number,
    sub_group_list: Department[],
    course: string,
    seq_num: number,
    title: string,
    long_title: string,
    attr_code: CourseAttribute[],
    units: string,
    cap: number,
    enr: number,
    permission_only: boolean,
    instructor: string,
    schedule: ClassPeriod[],
    location: ClassLocation[],
    note: string,
    taken_by_user?: boolean,
}

export type ClassPeriod = {
    days: CourseDay[],
    time: TimeRange
}

export type ClassLocation = string | undefined;

export type TimeRange = {
    start: ClassTime,
    end: ClassTime,
}

export type ClassTime = {
    hour: number,
    minute: number,
}

export type CourseAttribute = "AP" | "DI" | "FA" | "GP" | "HP" | "LP" | "POP" | "SP" | "VE" | "VP";


export type CourseDays = CourseDay[] | undefined;

export type CourseDay = "M" | "T" | "W" | "R" | "F";

export type Department = "ACCT" | "AFRC" | "ARAB" | "ARTH" | "ARTS" | "AS" | "ASTR" | "BAN" | "BCMB" | "BCOM" | "BIOL" | "BLAW" | "CHEM" | "CHIN" | "CLAS" | "CMLT" | "CRES" | "CRW" | "CSAC" | "CSCI" | "CYES" | "DSCI" | "ECON" | "EDUC" | "EN" | "ENG" | "ENT" | "EPP" | "FIN" | "FREN" | "GAME" | "GART" | "GCPT" | "GEOG" | "GERM" | "GES" | "GGRA" | "HEBR" | "HGS" | "HIST" | "HS" | "HSS" | "ID" | "IDCE" | "IDND" | "JAPN" | "JS" | "LALS" | "LAS" | "LAT" | "MATH" | "MCA" | "MGMT" | "MKT" | "MUSC" | "NRES" | "PECO" | "PHIL" | "PHYS" | "PSCI" | "PSYC" | "QBUS" | "SCRN" | "SOC" | "SPAN" | "TA" | "UDSC" | "WGS";