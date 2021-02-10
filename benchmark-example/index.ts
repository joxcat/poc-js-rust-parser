function processLinks(value: string[]): string[] {
    return value.flatMap((v, idxFlat) => typeof v === 'string'
        ? regexifyString(
            /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&/=]*)/gim,
            v,
            (matches, idxReg) =>
                <a key={`${idxFlat}-${idxReg}`} href={matches[0]} target="_blank" rel="noopener noreferrer">{matches[0]}</a>)
        : [v]);
}

export function regexifyString(pattern: RegExp, input: string, decorator: (matches: string[], index: number) => string | JSX.Element): Array<(string | JSX.Element)> {
    /* const {
        pattern,
        decorator,
        input,
    } = props;*/
    const output: Array<(string | JSX.Element)> = [];

    let matchIndex = 0;
    let processedInput = input;
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

export function processMentions(value: (string | JSX.Element)[], generateLink: boolean = true) {
    return value.flatMap(v => typeof v === 'string' ? regexifyString(/@\[(\w+)]\((\d+)\)/g, v, generateLink ? (matches, index) => {
        return <Link key={index} to={'connected.paanteon.user'}
                     params={{userId: matches[2], username: matches[1]}}>@{matches[1]}</Link>
    } : (matches) => {
        return `@${matches[1]}`
    }) : [v]);
}
