const wasmPromise = WebAssembly.instantiateStreaming(fetch("advent_of_code.wasm")).then(m => m.instance);
const encoder = new TextEncoder();
const decoder = new TextDecoder();

onmessage = async (e) => {
    const wasm = await wasmPromise;
    const parts_ptr = wasm.exports.alloc(8);
    const parts = new Uint32Array(wasm.exports.memory.buffer, parts_ptr, 2);
    const buf = encoder.encode(e.data[1]);
    const str_ptr = wasm.exports.alloc(buf.length);
    new Uint8Array(wasm.exports.memory.buffer, str_ptr, buf.length).set(buf);
    parts[0] = str_ptr;
    parts[1] = buf.length;
    const ok = wasm.exports.solve(e.data[0], parts_ptr);
    const output_parts = new Uint32Array(wasm.exports.memory.buffer, parts_ptr, 2);
    const output_buf = new Uint8Array(wasm.exports.memory.buffer, output_parts[0], output_parts[1]);
    const output = decoder.decode(output_buf);
    wasm.exports.dealloc(parts[0], parts[1]);
    wasm.exports.dealloc(parts_ptr, 8);
    postMessage([ok, output]);
}