import init, { run_testapp_app } from './pkg/serving.js';
async function main() {
   await init('/pkg/serving_bg.wasm');
   run_testapp_app();
}
main()
