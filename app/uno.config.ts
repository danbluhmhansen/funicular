import { defineConfig } from "unocss";
import presetUno from "@unocss/preset-uno";

export default defineConfig({
  presets: [
    presetUno({ dark: "media" }),
  ],
  cli: {
    entry: {
      patterns: ["src/**/*.rs"],
      outFile: "public/site.css",
    },
  },
});
