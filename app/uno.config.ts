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
        "svg-spinners": () => import("@iconify-json/svg-spinners/icons.json").then((i) => i.default),
        tabler: () => import("@iconify-json/tabler/icons.json").then((i) => i.default),
      },
    }),
    presetForms(),
  ],
  shortcuts: [
    [/^btn-(.*)$/, ([, c]) => `inline-flex items-center py-2 px-4 text-sm font-medium text-center text-${c}-600 bg-transparent rounded border border-${c}-600 dark:text-${c}-300 dark:border-${c}-300 hover:text-white hover:bg-${c}-500 focus:ring-4 focus:ring-${c}-400 focus:outline-none dark:hover:bg-${c}-400 dark:focus:ring-${c}-500`],
    {
      "btn-primary": "btn-violet",
      "btn-success": "btn-green",
      "btn-warning": "btn-amber",
      "btn-error": "btn-red",
    },
  ],
});
