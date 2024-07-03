#!/usr/bin/env node

import * as esbuild from 'esbuild'
import { polyfillNode } from "esbuild-plugin-polyfill-node";

const ctx = await esbuild.context({
    entryPoints: ["lib.js"],
    outfile: "bundle.js",
    platform: "browser",
    format: "esm",
    bundle: true,
    plugins: [
        polyfillNode({}),
    ],
});

// TODO: improve watch mode feedback on rebuilding
await ctx.watch();
console.log("watching...");
