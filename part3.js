
const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout,
});

function isNumber(char) {
    if (typeof char !== 'string') {
        return false;
    }

    if (char.trim() === '') {
        return false;
    }

    return !isNaN(char);
}


// returns list of tokens
function tokenize(source) {
    let ret = [];

    for (let i = 0; i < source.length; i++) {
        if (source[i] == " ") {
            continue;
        }

        if (isNumber(source[i])) {
            let endIndex = i;

            while (isNumber(source[endIndex])){
                endIndex++;
            }

            let value = parseInt(source.slice(i, endIndex));

            ret.push({
                type: "Number",
                value
            });

            i = endIndex - 1;
            continue;
        }

        if (source[i] == "+") {
            ret.push({
                type: "Plus",
                value: "+"
            });
            continue;
        }

        if (source[i] == "-") {
            ret.push({
                type: "Minus",
                value: "-"
            });
            continue;
        }

        throw new Error("Syntax error");
    }

    return ret;
}

function parser(tokens) {
    if (tokens.length == 0) return;

    if (tokens[0].type != "Number") {
        throw new Error("Parser error: expected int");
    }

    let result = tokens[0].value;

    for (let i = 1; i < tokens.length; i++) {
        let op = tokens[i].type;
        if (!["Plus", "Minus"].includes(op)) {
            throw new Error("Parser error: expected plus or minus");
        }

        i++;

        if (i >= tokens.length) {
            throw new Error("Parser error: expected number");
        }

        if (tokens[i].type != "Number") {
            throw new Error("Parser error: expected int");
        }

        let value = tokens[i].value;

        if (op == "Plus") {
            result += value;
        } else if (op == "Minus") {
            result -= value;
        }

    }

    return result;
}

function readInput() {
    process.stdout.write("calc> ");

    readline.question(`calc> `, src => {
        let tokens = tokenize(src);
        let result = parser(tokens);
        console.log(result);

        readInput();
    });
}

readInput();
