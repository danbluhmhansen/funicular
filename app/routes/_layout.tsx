import { defineLayout } from "$fresh/server.ts";

export default defineLayout((_, { Component }) => {
  return (
    <div class="min-h-screen dark:text-white dark:bg-slate-900">
      <nav class="py-4">
        <ul class="flex flex-col gap-4 justify-center items-center sm:flex-row">
          <li>
            <a href="/" class="data-[current]:text-violet hover:text-violet">
              Home
            </a>
          </li>
          <li>
            <a
              href="/games"
              class="data-[current]:text-violet hover:text-violet"
            >
              Games
            </a>
          </li>
        </ul>
      </nav>
      <main class="container flex flex-col gap-4 justify-center items-center mx-auto">
        <Component />
      </main>
    </div>
  );
});
