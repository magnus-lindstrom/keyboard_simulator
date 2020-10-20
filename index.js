// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/keyboard_optimiser.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const keyboard_optimiser = await init("./pkg/keyboard_optimiser_bg.wasm");

  // Call the Add function export from wasm, save the result
  keyboard_optimiser.start();

  // Set the result onto the body
  document.body.textContent = `Hello World! new update addResult: ${addResult}`;
};
runWasm();
