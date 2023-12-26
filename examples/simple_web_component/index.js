import init from "./pkg/simple_web_component.js";

const runWasm = async () => {
    await init("./pkg/simple_web_component_bg.wasm");
};
runWasm();