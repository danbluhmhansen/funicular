import { JSX } from "preact";
import { IS_BROWSER } from "$fresh/runtime.ts";

export function Button(props: JSX.HTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      disabled={!IS_BROWSER || props.disabled}
      class={`
        rounded-md px-4 py-2
        flex flex-row items-center justify-center
        text-(white sm) font-medium
        bg-blue-(700 hover:800 active:900 dark:600 dark:hover:700 dark:active:800)
        focus-(ring-4 ring-blue-300 outline-none dark:ring-blue-800)
      `}
    />
  );
}
