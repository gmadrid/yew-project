import init, { run_twocolor_app } from './pkg/serving.js';
async function main() {
   await init('/pkg/serving_bg.wasm');
   run_twocolor_app();
}
main()
