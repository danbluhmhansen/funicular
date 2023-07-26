import { defineConfig, presetIcons, presetTypography, presetUno } from "unocss";
import { presetDaisy } from "unocss-preset-daisy";

export default defineConfig({
  presets: [
    presetUno(),
    presetIcons(),
    presetTypography(),
    presetDaisy(),
  ],
});
