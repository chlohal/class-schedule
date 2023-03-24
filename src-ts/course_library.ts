import { Course } from "./course_types.js"

export class CourseLibrary {
    courses: Course[];
    course_name_index: Partial<Record<string, Course[]>>;
    
    constructor() {
        this.courses = [];
        this.course_name_index = {};
    }

    *[Symbol.iterator]() {
        for (const c of this.courses) yield c;
    }

    getByName(name: string) {
        if(!(name.toUpperCase() in this.course_name_index)) console.warn(`Class ${name} is not in the course library`);

        return this.course_name_index[name.toUpperCase()];
    }

    filterCourses(filterFunction: (_: Course) => any) {
        this.courses = this.courses.filter(filterFunction);
    }

    addCourse(...courses: Course[]) {
        for (const c of courses) {
            this.courses.push(c);

            if(c.course in this.course_name_index) {
                this.course_name_index[c.course.toUpperCase()]?.push(c);
            } else {
                this.course_name_index[c.course.toUpperCase()] = [c];
            }
        }
    }
}