import { JSX } from "preact";

export function Button(props: JSX.HTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      {...props}
      class="inline-block py-2 px-4 text-sm font-medium text-center text-violet-600 bg-transparent rounded border border-violet-600 dark:text-violet-300 dark:border-violet-300 hover:text-white hover:bg-violet-500 focus:ring-4 focus:ring-violet-400 focus:outline-none dark:hover:bg-violet-400 dark:focus:ring-violet-500"
    />
  );
}
