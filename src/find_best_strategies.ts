import { ProgramArgs } from "./args.js";
import { Course, Strategy } from "./course_types.js";
import evaluateStrategy from "./evaluate_strategy.js";
import scheduleOverlaps from "./schedule_overlaps.js";

const MAX_CLASSES = 4;

export default function findBestStrategies(args: ProgramArgs) {
    let baseQueue = [new Set()];

    let bestOverall: Set<Set<Course>> = new Set();

    while (baseQueue.length) {
        const base = baseQueue.shift() as Set<Course>;
        const best = findBestStratsFromBase(base, args);
        for (const b of best) {
            baseQueue.push(b);
            bestOverall.add(b);
        }
        console.warn("In queue: " + baseQueue.length);
    }

    return Array.from(bestOverall)
        .sort((a, b) => evaluateStrategy(b, args.taken) - evaluateStrategy(a, args.taken))
        .map(x=>Array.from(x));
}

function findBestStratsFromBase(base: Set<Course>, args: ProgramArgs): Set<Set<Course>> {
    if (base.size >= MAX_CLASSES) return new Set;

    let bestScore = evaluateStrategy(base, args.taken);;
    const bests: Set<Set<Course>> = new Set();

    for (const course of args.courses) {
        if(scheduleOverlaps(base, course)) continue;
        if(args.taken.find(x=>x.course == course.course)) continue;

        const tryStrat = new Set(base);
        tryStrat.add(course);

        const score = evaluateStrategy(tryStrat, args.taken);

        if (score > bestScore) {
            //bests.clear();
            bests.add(tryStrat);
            bestScore = score;
        }
        else if (score == bestScore) {
            bests.add(tryStrat);
        }
    }

    return bests;
}