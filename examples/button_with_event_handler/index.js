import init from "./pkg/button_with_event_handler.js";

const runWasm = async () => {
    await init("./pkg/button_with_event_handler_bg.wasm");
};
runWasm();