import { readFileSync } from "fs";
import { Goal } from "./course_types.js";

export function load_goals(filename: string): Goal[] {
    return JSON.parse(readFileSync(filename).toString()) as Goal[];
}