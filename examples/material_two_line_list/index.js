import init from "./pkg/material_two_line_list.js";

const runWasm = async () => {
    await init("./pkg/material_two_line_list_bg.wasm");
};
runWasm();