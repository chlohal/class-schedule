import { readFileSync } from "fs";

const pageContent = readFileSync(process.argv[2]).toString();

const DIV_START_SIGIL = "<div>";
const DIV_END_SIGIL = "</div>";

const contentStartIndex = pageContent.lastIndexOf(DIV_START_SIGIL);
const contentEndIndex = pageContent.indexOf(DIV_END_SIGIL, contentStartIndex);

const content = pageContent.substring(contentStartIndex + DIV_START_SIGIL.length, contentEndIndex);

if (!content) throw new Error("Unable to extract content!");

const heading = extractContentsOfNonNestedElements(content, "h3")[0];

console.log(heading);

const description = getRidOfHTML(
    content.substring(content.indexOf("<hr>") + "<hr>".length)
        .replace(/<\/?p>/g, "\n")
        .replace(/<br>/g, "\n")
        .replace(/&#160;/g, " ")
        .replace(/\n+/g, "\n")
);

console.log(description);

function getRidOfHTML(text) {
    let result = "";

    let doubleQuote = false, angleBracket = false;
    for (const char of text) {
        if (char == "<" && !doubleQuote) angleBracket = true;
        if (!angleBracket) result += char;
        if (char == ">" && !doubleQuote) angleBracket = false;

        if (char == "\"") doubleQuote = !doubleQuote;
    }

    return result;
}

function extractContentsOfNonNestedElements(content, tagName) {
    const startSigil = `<${tagName}>`;
    const endSigil = `</${tagName}>`;

    const elems = [];

    let lastEnd = -1;
    while (true) {
        const startIndex = content.indexOf(startSigil, lastEnd);
        if (startIndex == -1) break;

        const endIndex = content.indexOf(endSigil, startIndex);

        elems.push(content.substring(startIndex + startSigil.length, endIndex));
        lastEnd = endIndex + endSigil.length;
    }

    return elems;
}