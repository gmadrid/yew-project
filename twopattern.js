import init, { run_twopattern_app } from './pkg/twopattern.js';
async function main() {
   await init('/pkg/twopattern_bg.wasm');
   run_twopattern_app();
}
main()
