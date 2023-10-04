import { JSX } from "preact";

export function Button(props: JSX.HTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      class="
        bg-transparent hover:bg-violet-500 dark:hover:bg-violet-400
        border border-violet-600 dark:border-violet-300
        font-medium
        focus:outline-none
        px-4 py-2
        focus:ring-4 focus:ring-violet-400 dark:focus:ring-violet-500
        rounded
        text-sm text-center hover:text-white text-violet-600 dark:text-violet-300
      "
    />
  );
}
