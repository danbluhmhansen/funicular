import { defineConfig, presetUno } from "unocss";
import presetIcons from "@unocss/preset-icons";
import { presetForms } from "@julr/unocss-preset-forms";

export default defineConfig({
  cli: {
    entry: {
      outFile: "assets/site.css",
      patterns: ["src/**/*.rs"],
    },
  },
  presets: [
    presetUno({ dark: "media" }),
    presetIcons({
      collections: {
        tabler: () =>
          import("@iconify-json/tabler/icons.json").then((i) => i.default),
      },
    }),
    presetForms(),
  ],
});
