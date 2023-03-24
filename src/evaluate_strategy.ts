import { Course, CourseAttribute, CourseCredit, CourseDay, Department, Goal, GoalField, GoalTarget, GoalTargetValue, Strategy } from "./course_types.js";

export default function evaluateStrategy(strat: Set<Course>, alreadyHad: CourseCredit[]): number {
    let days: Set<CourseDay> = new Set();
    let VE = false, FA = false, DI = false;
    let uniqAttrs: Partial<Record<Department, CourseAttribute>> = {};
    let attrs: Set<CourseAttribute> = new Set();

    for (const c of alreadyHad) {
        const dept = c.course.split(" ")[0] as Department;

        for (const a of c.attr_code) {
            addAttribute(a, uniqAttrs, attrs, dept);
        }
    }

    

    for (const c of strat) {
        if (alreadyHad.find(x => x.course == c.course)) return -Infinity;

        if (c.note) {
            const noteTerms = c.note.split(";").map(x => x.trim());

            if (noteTerms.includes("JRS & SRS ONLY") || noteTerms.includes("JR/SR ONLY")) return -Infinity;
        }

        for (const day of c.schedule.flatMap(x => x.days)) days.add(day);

        const dept = c.course.split(" ")[0] as Department;
        for (const a of c.attr_code) {
            addAttribute(a, uniqAttrs, attrs, dept);
        }
    }

    return -1 / 5 * days.size
        + 1 / 7 * (
            [VE, FA, DI].filter(x => x).length + attrs.size
        );
}

function addAttribute(a: CourseAttribute, uniqAttrs: Partial<Record<Department, CourseAttribute>>, attrs: Set<CourseAttribute>, dept: Department) {
    if (a == "VE" || a == "FA" || a == "DI") {
        attrs.add(a);
    } else {
        if (a.match(/^[AGHLSV]P$/)) {
            if (uniqAttrs[dept] == undefined) {
                uniqAttrs[dept] = a;
                attrs.add(a);
            }
        }
    }
}
