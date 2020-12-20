import init, { run_metapixel_app } from './pkg/serving.js';
async function main() {
   await init('/pkg/serving_bg.wasm');
   run_metapixel_app();
}
main()
