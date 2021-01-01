import init, { run_tiles_app } from './pkg/serving.js';
async function main() {
   await init('/pkg/serving_bg.wasm');
   run_tiles_app();
}
main()
