import {
  createGenerator,
  type UserConfig,
} from "https://esm.sh/@unocss/core@0.55.0";
import { Plugin } from "$fresh/server.ts";

export default function unocss(config: UserConfig): Plugin {
  return {
    name: "unocss",
    async renderAsync({ renderAsync }) {
      const { htmlText } = await renderAsync();
      const { css } = await createGenerator(config).generate(htmlText, {
        minify: true,
      });
      return {
        styles: [{ cssText: css }],
      };
    },
  };
}
