import rust from "@wasm-tool/rollup-plugin-rust";
import typescript from "@rollup/plugin-typescript";

export default {
    input: "index.ts",
    output: {
        file: "dist/fluvio-client.js",
        format: "umd",
        sourcemap: true,
	name: "fluvio",
    },
    plugins: [
        rust({
	    watchPatterns: ["src/**"],
	}),
	typescript()
    ],
};
