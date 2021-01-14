import init, { run_PROJECT_app } from './pkg/PROJECT.js';
async function main() {
   await init('/pkg/PROJECT_bg.wasm');
   run_PROJECT_app();
}
main()
