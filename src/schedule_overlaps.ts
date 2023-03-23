import { ClassPeriod, Course, CourseDay } from "./course_types.js";

export default function scheduleOverlaps(schedule: Set<Course>, newCourse: Course): boolean {
    for(const course of schedule) {
        if(classesOverlap(course.schedule, newCourse.schedule)) return true;
    }
    return false;
}

function classesOverlap(a: ClassPeriod[], b: ClassPeriod[]): boolean {
    for(const [aRangeMin, aRangeMax] of minuteRanges(a)) {
        for(const [bRangeMin, bRangeMax] of minuteRanges(b)) {
            if(aRangeMin <= bRangeMax && aRangeMax >= bRangeMin) return true;
            if(bRangeMin <= aRangeMax && bRangeMax >= aRangeMin) return true;


            if(aRangeMin >= bRangeMin && aRangeMax <= bRangeMax) return true;
            if(bRangeMin >= aRangeMin && bRangeMax <= aRangeMax) return true;
        }
    }
    return false;

    
}

function* minuteRanges(periods: ClassPeriod[]) {
    for(const period of periods) {
        const t = period.time;
        for(const d of period.days) {
            yield [
                dayNumber(d) + t.start.hour * 60 + t.start.minute,
                dayNumber(d) + t.end.hour * 60 + t.end.minute
            ];
        }
    }
}

function dayNumber(d: CourseDay): number {
    return {
        "M": 0,
        "T": 1,
        "W": 2,
        "R": 3,
        "F": 4
    }[d] * 24 * 60;
}