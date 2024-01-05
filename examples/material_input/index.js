import init from "./pkg/material_input.js";

const runWasm = async () => {
    await init("./pkg/material_input_bg.wasm");
};
runWasm();