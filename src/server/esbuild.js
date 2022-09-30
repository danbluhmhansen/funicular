#!/usr/bin/env node
require('esbuild').build({
  logLevel: 'info',
  entryPoints: [
    "Content/pages/home/index.tsx",
  ],
  bundle: true,
  outdir: 'wwwroot/dist',
  outbase: 'Content/pages',
}).catch(() => process.exit(1));
