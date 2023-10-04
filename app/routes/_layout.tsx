import { defineLayout } from "$fresh/server.ts";

export default defineLayout((_, { Component }) => {
  return (
    <div class="min-h-screen dark:bg-slate-900 dark:text-white">
      <nav class="py-4">
        <ul class="flex flex-col sm:flex-row items-center justify-center gap-4">
          <li>
            <a href="/" class="hover:text-violet">Home</a>
          </li>
          <li>
            <a href="/games" class="hover:text-violet">Games</a>
          </li>
        </ul>
      </nav>
      <main class="container mx-auto flex flex-col items-center justify-center gap-4">
        <Component />
      </main>
    </div>
  );
});
