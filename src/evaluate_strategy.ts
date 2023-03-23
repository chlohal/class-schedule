import { Course, CourseAttribute, CourseCredit, CourseDay, Department, Goal, GoalField, GoalTarget, GoalTargetValue, Strategy } from "./course_types.js";

export default function evaluateStrategy(strat: Set<Course>, alreadyHad: CourseCredit[]): number {
    let days: Set<CourseDay> = new Set();
    let VE = false, FA = false, DI = false;
    let uniqAttrs: Partial<Record<Department, CourseAttribute>> = {};
    let hasAttrs: Set<CourseAttribute> = new Set();

    for(const c of strat) {
        if(alreadyHad.find(x=>x.course == c.course)) return -Infinity;
    }

    for (const c of alreadyHad.concat(Array.from(strat)) as (Course | CourseCredit)[]) {

        if ("schedule" in c) {

            if (c.note) {
                const noteTerms = c.note.split(";").map(x => x.trim());

                if (noteTerms.includes("JRS & SRS ONLY")) return -Infinity;
            }

            for (const day of c.schedule.flatMap(x => x.days)) days.add(day);
        }

        const dept = c.course.split(" ")[0];
        for (const a of c.attr_code) {
            if (a == "VE") VE = true;
            if (a == "FA") FA = true;
            if (a == "DI") DI = true;

            if (!hasAttrs.has(a)) {
                if (a.match(/^[AGHLSV]P$/)) {
                    if (!(dept in uniqAttrs)) {
                        uniqAttrs[dept as Department] = a;
                        hasAttrs.add(a);
                    }
                }
            }
        }
    }

    return -3 * days.size
        + 7 * (
            [VE, FA, DI].filter(x => x).length + hasAttrs.size
        );
}