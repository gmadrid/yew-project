import init, { run_twocolor_app } from './pkg/colortoo.js';
async function main() {
   await init('/pkg/colortoo_bg.wasm');
   run_testapp_app();
}
main()
