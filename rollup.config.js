import rust from "@wasm-tool/rollup-plugin-rust";
import typescript from "@rollup/plugin-typescript";
import dts from "rollup-plugin-dts";

const config = [
  {
    input: "index.ts",
    output: {
      file: "dist/fluvio-client.js",
      format: "umd",
      sourcemap: true,
      name: "fluvio",
    },
    plugins: [
      rust({
	  serverPath: "/",
      }),
      typescript(),
    ],
  },
  {
    input: "index.ts",
    output: [{ file: "dist/fluvio-client.d.ts", format: "es" }],
    plugins: [rust(), dts()],
  },
];

export default config;
