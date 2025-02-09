import initSync, * as wasm from "./pkg/wasm_integration.js";
const form = document.getElementsByTagName("form")[0];

try {
  await initSync();
  console.log("WASM Loaded");

  form.onsubmit = async (e) => {
    e.preventDefault();

    if (!wasm) {
      await initSync();
      throw new Error("No WASM was found!");
    }

    let n = parseInt(document.getElementById("input").value);
    document.getElementById("result").innerText = wasm?.fibonacci(n);
  };
} catch (err) {
  console.log("An error has occured\n", err);
}
