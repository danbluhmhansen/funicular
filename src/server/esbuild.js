#!/usr/bin/env node
const { build } = require("esbuild");
const glob = require("tiny-glob");

(async () => {
  let entryPoints = await glob("Content/pages/**/*.tsx");
  await build({
    entryPoints,
    logLevel: 'info',
    bundle: true,
    outdir: 'wwwroot/dist',
  });
})();
