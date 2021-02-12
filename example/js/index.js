import("../pkg/index.js").catch(console.error);

function regexifyString(pattern: RegExp, input: string, decorator: (matches: string[], index: number) => string): Array<string> {
    const output: Array<string> = [];

    let matchIndex = 0;
    let processedInput: string = input;
    let result = pattern.exec(processedInput);

    while (result !== null) {
        const matchStartAt = result.index;
        const match = result;

        const contentBeforeMatch = processedInput.substring(0, matchStartAt);
        const decoratedMatch = decorator(match, matchIndex);

        output.push(contentBeforeMatch);
        output.push(decoratedMatch);

        // clear processed content: before match, match
        processedInput = processedInput.substring(matchStartAt + match[0].length, processedInput.length + 1);

        pattern.lastIndex = 0;

        result = pattern.exec(processedInput);

        ++matchIndex;
    }

    if (processedInput) {
        output.push(processedInput);
    }

    return output;
}

function processLinks(value: string[]): string[] {
    return value.flatMap((v, idxFlat) => regexifyString(
        /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&/=]*)/gim,
        v,
        (matches, idxReg) =>
            `<a data-key="${idxFlat}-${idxReg}" href="${matches[0]}" target="_blank" rel="noopener noreferrer">${matches[0]}</a>`));
}

function processMentions(value: string[], generateLink: boolean = true) {
    return value.flatMap(v => regexifyString(/@\[(\w+)]\((\d+)\)/g, v, generateLink ? (matches, index) => {
        return `<a data-key="${index}" to="connected.paanteon.user" data-params="${{userId: matches[2], username: matches[1]}}">${matches[1]}</a>`
    } : (matches) => {
        return `@${matches[1]}`
    }));
}


const x = parse_demo("test @[some](2) https://asdasd.com");

console.log(x);
