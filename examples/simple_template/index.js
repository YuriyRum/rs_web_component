import init from "./pkg/simple_template.js";

const runWasm = async () => {
    await init("./pkg/simple_template_bg.wasm");
};
runWasm();