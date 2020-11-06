import init, { run_app } from './pkg/colortoo.js';
async function main() {
   await init('/pkg/colortoo_bg.wasm');
   run_app();
}
main()
