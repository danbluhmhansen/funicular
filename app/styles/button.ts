export function Button(
  color: "violet" | "green" | "yellow" | "red" = "violet",
) {
  return `inline-block py-2 px-4 text-sm font-medium text-center text-${color}-600 bg-transparent rounded border border-${color}-600 dark:text-${color}-300 dark:border-${color}-300 hover:text-white hover:bg-${color}-500 focus:ring-4 focus:ring-${color}-400 focus:outline-none dark:hover:bg-${color}-400 dark:focus:ring-${color}-500`;
}
