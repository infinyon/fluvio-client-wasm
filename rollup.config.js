import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        fluvio_client: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust(),
    ],
};
