import init, { run_crown_app } from './pkg/serving.js';
async function main() {
   await init('/pkg/serving_bg.wasm');
   run_crown_app();
}
main()
