const worker = new Worker("worker.js");
worker.onmessage = (e) => {
    if (e.data[0]) {
        document.getElementById("output").value = e.data[1];
    } else {
        document.getElementById("error").textContent = e.data[1];
    }
    document.getElementById("solve").classList.remove("inProgress");
};
function exec() {
    document.getElementById("output").value = "";
    document.getElementById("error").textContent = "";
    document.getElementById("solve").classList.add("inProgress");
    worker.postMessage([document.getElementById("day").value, document.getElementById("input").value]);
}