import { defineConfig } from "$fresh/server.ts";
import unocssPlugin from "./plugins/unocss.ts";
import presetUno from "https://esm.sh/@unocss/preset-uno@0.55.0";
import { presetForms } from "https://esm.sh/@julr/unocss-preset-forms@0.0.5";
import presetIcons from "https://esm.sh/@unocss/preset-icons@0.55.0";

export default defineConfig({
  port: 1111,
  plugins: [unocssPlugin({
    presets: [
      presetUno({ dark: "media" }),
      presetForms(),
      presetIcons({
        collections: {
          tabler: () => import("https://esm.sh/@iconify-json/tabler@1.1.89/icons.json").then((i) => i.default),
        },
      }),
    ],
  })],
});