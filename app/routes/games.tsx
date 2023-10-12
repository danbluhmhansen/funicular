import { defineRoute, Handlers } from "$fresh/server.ts";
import { signal } from "@preact/signals";
import { SERVER_URL } from "~utils/env.ts";
import { Button } from "~styles/button.ts";
import Checkbox from "~islands/checkbox.tsx";

interface Game {
  name: string;
  slug: string;
}

export const handler: Handlers = {
  async POST(req, { render }) {
    const form = await req.formData();

    const submit = form.get("submit")?.toString();
    form.delete("submit");

    switch (submit) {
      case "add": {
        await fetch(`${SERVER_URL}/game`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(Object.fromEntries(form)),
        });
        break;
      }
      case "remove": {
        const slugs = form.getAll("slug").map((entry) => entry.toString()).join(
          ",",
        );
        await fetch(`${SERVER_URL}/game?slug=in.(${slugs})`, {
          method: "DELETE",
        });
        break;
      }
    }

    return await render();
  },
};

export default defineRoute(async () => {
  const response = await fetch(
    `${SERVER_URL}/game?select=name,slug`,
  );
  const games: Game[] = await response.json();

  const checked = signal(false);

  return (
    <>
      <h1 class="text-xl font-bold">Games</h1>
      <div class="overflow-x-auto relative rounded shadow-md">
        <form method="post">
          <div class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800">
            <a
              href="#add"
              class={Button()}
            >
              <div class="w-4 h-4 i-tabler-plus" />
            </a>
            <button
              type="submit"
              name="submit"
              value="remove"
              class={Button("red")}
            >
              <div class="w-4 h-4 i-tabler-trash" />
            </button>
          </div>
          <table class="w-full">
            <thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700">
              <tr>
                <th class="p-3 text-center">
                  <Checkbox checked={checked} class="bg-transparent" />
                </th>
                <th class="p-3 text-center">Name</th>
              </tr>
            </thead>
            <tbody>
              {games.map((game) => (
                <tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700">
                  <td class="p-3 text-center">
                    <Checkbox
                      name="slug"
                      value={game.slug}
                      checked={checked}
                      noSet
                      class="bg-transparent"
                    />
                  </td>
                  <td class="p-3 text-center">
                    <a href={`/games/${game.slug}`} class="hover:text-violet">
                      {game.name}
                    </a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </form>
      </div>
      <dialog
        id="add"
        class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
      >
        <div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900">
          <div>
            <a href="#!" class="float-right w-4 h-4 i-tabler-x" />
            <h2 class="text-xl">Add Game</h2>
          </div>
          <form method="post" class="flex flex-col gap-4 justify-center">
            <input
              type="text"
              name="name"
              placeholder="Name"
              required
              autofocus
              class="bg-transparent rounded invalid:border-red"
            />
            <textarea
              name="description"
              placeholder="Description"
              class="bg-transparent rounded invalid:border-red"
            />
            <div class="flex justify-between">
              <button type="submit" name="submit" value="add" class={Button()}>
                <div class="w-4 h-4 i-tabler-check" />
              </button>
            </div>
          </form>
        </div>
        <a href="#!" class="fixed inset-0" />
      </dialog>
    </>
  );
});
