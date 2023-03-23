import { args_parsed } from "./args.js";
import { Course, courseToString } from "./course_types.js";
import findBestStrategies from "./find_best_strategies.js";

const args = args_parsed();

if (process.argv.length < 3) args.extra.help = "";

if (!("include-unspecified" in args.extra)) {
    args.courses.filterCourses(course =>
        course.schedule.length > 0
    );
}

if ("show" in args.extra) {
    console.info(args.courses.getByName(args.extra.show as string));
} else if ("search" in args.extra) {
    try {
        console.info(args.courses.courses.filter(new Function("x", `return !!(${args.extra.search})`) as (x: Course) => boolean));
    } catch (e) {
        console.error("Error in 'search' syntax");
    }
} else if ("help" in args.extra) {
    console.info(`Course Scheduler!
    Arguments:
    --courses-file=/path/to/file.json
    --courses-taken-file=/path/to/file.json
    --goals-file=/path/to/file.json
    --include-unspecified
    --show=course-name
    --search=filter-function
    `)
} else {
    const best = findBestStrategies(args);

    if (args.extra.output == "json") {
        console.log(JSON.stringify(best));
    } else if (args.extra.output == "pretty-json") {
        console.log(JSON.stringify(best, null, 2));
    } else {
        for (const strat of best) {
            console.log(strat.map(x => courseToString(x)).join(", "))
        }
    }
}