export * from "./target/wasm-pack/fluvio-client-wasm/index";

export type Exports = typeof import("./target/wasm-pack/fluvio-client-wasm/index");
declare const init: () => Promise<Exports>;
export default init;
