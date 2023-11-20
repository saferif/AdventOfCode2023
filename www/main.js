const worker1 = new Worker("worker.js");
const worker2 = new Worker("worker.js");

function solveAsync(worker, index, input) {
    return new Promise((resolve, reject) => {
        worker.onmessage = e => {
            (e.data[0] ? resolve : reject)(e.data[1]);
        };
        worker.postMessage([index, input]);
    });
}

function prepareOutput(e) {
    e.value = "";
    e.classList.remove("error");
}

function handleOutput(p, e) {
    return p.then(output => {
        e.value = output;
    }).catch(reason => {
        e.value = reason;
        e.classList.add("error");
    });
}

function exec() {
    const output1 = document.getElementById("output1");
    const output2 = document.getElementById("output2");
    const solveBtn = document.getElementById("solve");
    prepareOutput(output1);
    prepareOutput(output2);
    solveBtn.classList.add("inProgress");
    document.body.style.cursor = "wait";
    const day = parseInt(document.getElementById("day").value) - 1;
    const input = document.getElementById("input").value;
    const part1 = handleOutput(solveAsync(worker1, 2 * day, input), output1);
    const part2 = handleOutput(solveAsync(worker2, 2 * day + 1, input), output2);
    return Promise.all([part1, part2]).finally(() => {
        solveBtn.classList.remove("inProgress");
        document.body.style.cursor = "default";
    });
}