import { defineRoute, Handlers } from "$fresh/server.ts";
import { Button } from "../components/Button.tsx";

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
        await fetch("http://localhost:3000/game", {
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
        await fetch(`http://localhost:3000/game?slug=in.(${slugs})`, {
          method: "DELETE",
        });
        break;
      }
    }
    return await render();
  },
};

export default defineRoute(async () => {
  const response = await fetch("http://localhost:3000/game?select=name,slug");
  const games: Game[] = await response.json();
  return (
    <>
      <h1 class="text-xl font-bold">Games</h1>
      <div class="overflow-x-auto relative rounded shadow-md">
        <form method="post">
          <div class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800">
            <a
              href="#add"
              class="inline-block py-2 px-4 text-sm font-medium text-center text-violet-600 bg-transparent rounded border border-violet-600 dark:text-violet-300 dark:border-violet-300 hover:text-white hover:bg-violet-500 focus:ring-4 focus:ring-violet-400 focus:outline-none dark:hover:bg-violet-400 dark:focus:ring-violet-500"
            >
              <div class="w-4 h-4 i-tabler-plus" />
            </a>
            <Button type="submit" name="submit" value="remove">
              <div class="w-4 h-4 i-tabler-trash" />
            </Button>
          </div>
          <table class="w-full">
            <thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700">
              <tr>
                <th class="p-3 text-center">
                  {/* TODO: select all */}
                  <input type="checkbox" class="bg-transparent" />
                </th>
                <th class="p-3 text-center">Name</th>
              </tr>
            </thead>
            <tbody>
              {games.map((game) => (
                <tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700">
                  <td class="p-3 text-center">
                    <input
                      type="checkbox"
                      name="slug"
                      value={game.slug}
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
              <Button type="submit" name="submit" value="add">
                <div class="w-4 h-4 i-tabler-check" />
              </Button>
            </div>
          </form>
        </div>
        <a href="#!" class="fixed inset-0" />
      </dialog>
    </>
  );
});
