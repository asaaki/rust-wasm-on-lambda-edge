import nodeBuiltins from 'builtin-modules'
import typescript from 'rollup-plugin-typescript2'
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import replace from "@rollup/plugin-replace";
import { terser } from "rollup-plugin-terser";

const external = nodeBuiltins;

// the wasm-bindgen output uses this escaped require call,
// but rollup and friends cannot deal with it, so we have to simplify again
const escapedUtil = "String.raw`util`";
const replaceUtil = '"util"';

const plugins = [
  replace({ [escapedUtil]: replaceUtil, delimiters: ["", ""], preventAssignment: true }),
  typescript({ tsconfig: "./tsconfig-build.json" }),
  resolve(),
  commonjs(),
  terser(),
];

const makeEntryPointFor = (input) => {
  return {
    input,
    output: {
      dir: "../build.function",
      format: "cjs",
      compact: true,
      interop: "defaultOnly",
      esModule: false,
      exports: "named",
      indent: false,
      preferConst: true,
    },
    external,
    plugins,
  };
}

export default [
  makeEntryPointFor('./entrypoints/origin-request.ts'),
]
