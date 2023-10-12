import { defineRoute, Handlers } from "$fresh/server.ts";
import { SERVER_URL } from "~utils/env.ts";
import { Button } from "~styles/button.ts";
import Checkbox from "~islands/checkbox.tsx";
import { signal } from "@preact/signals";

interface Game {
  id: string;
  name: string;
  slug: string;
  description?: string;
}

interface ActorKind {
  name: string;
  slug: string;
}

export const handler: Handlers = {
  async POST(req, { render, params: { gameSlug } }) {
    const form = await req.formData();

    const submit = form.get("submit")?.toString();
    form.delete("submit");

    switch (submit) {
      case "edit": {
        const res = await fetch(`${SERVER_URL}/game?slug=eq.${gameSlug}`, {
          method: "PATCH",
          headers: {
            Accept: "application/vnd.pgrst.object+json",
            "Content-Type": "application/json",
            Prefer: "return=representation",
          },
          body: JSON.stringify(Object.fromEntries(form)),
        });
        const { slug }: Game = await res.json();
        if (gameSlug !== slug) {
          return new Response(null, {
            status: 308,
            headers: { location: `/games/${slug}` },
          });
        }
        break;
      }
      case "add_actor": {
        await fetch(`${SERVER_URL}/actor_kind`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(Object.fromEntries(form)),
        });
        break;
      }
      case "remove_actor": {
        const slugs = form.getAll("slug").map((entry) => entry.toString()).join(
          ",",
        );
        await fetch(`${SERVER_URL}/actor_kind?slug=in.(${slugs})`, {
          method: "DELETE",
        });
        break;
      }
    }

    return await render();
  },
};

export default defineRoute(async (_, { params: { gameSlug } }) => {
  const resGame = await fetch(
    `${SERVER_URL}/game?slug=eq.${gameSlug}&select=id,name,slug,description`,
    { headers: { Accept: "application/vnd.pgrst.object+json" } },
  );
  const game: Game = await resGame.json();

  const resActorKinds = await fetch(
    `${SERVER_URL}/actor_kind?game.slug=eq.${gameSlug}&select=name,slug,game!inner()`,
  );
  const actorKinds: ActorKind[] = await resActorKinds.json();

  const checked = signal(false);

  return (
    <>
      <div class="flex flex-row gap-2 justify-center items-center">
        <h1 class="text-xl font-bold">{game.name}</h1>
        <a href="#edit" class={Button("yellow")}>
          <div class="w-4 h-4 i-tabler-pencil" />
        </a>
      </div>

      <div class="overflow-x-auto relative rounded shadow-md">
        <form method="post">
          <div class="flex flex-row gap-2 justify-center p-3 bg-white dark:bg-slate-800">
            <a
              href="#add_actor"
              class={Button()}
            >
              <div class="w-4 h-4 i-tabler-plus" />
            </a>
            <button
              type="submit"
              name="submit"
              value="remove_actor"
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
              {actorKinds.map((actorKind) => (
                <tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700">
                  <td class="p-3 text-center">
                    <Checkbox
                      name="slug"
                      value={actorKind.slug}
                      checked={checked}
                      noSet
                      class="bg-transparent"
                    />
                  </td>
                  <td class="p-3 text-center">
                    <a
                      href={`/games/${gameSlug}/actors/${actorKind.slug}`}
                      class="hover:text-violet"
                    >
                      {actorKind.name}
                    </a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </form>
      </div>

      <dialog
        id="edit"
        class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
      >
        <div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900">
          <div>
            <a href="#!" class="float-right w-4 h-4 i-tabler-x" />
            <h2 class="text-xl">Edit Game</h2>
          </div>
          <form method="post" class="flex flex-col gap-4 justify-center">
            <input
              type="text"
              name="name"
              placeholder="Name"
              required
              autofocus
              value={game.name}
              class="bg-transparent rounded invalid:border-red"
            />
            <textarea
              name="description"
              placeholder="Description"
              value={game.description}
              class="bg-transparent rounded invalid:border-red"
            />
            <div class="flex justify-between">
              <button type="submit" name="submit" value="edit" class={Button()}>
                <div class="w-4 h-4 i-tabler-check" />
              </button>
            </div>
          </form>
        </div>
        <a href="#!" class="fixed inset-0" />
      </dialog>

      <dialog
        id="add_actor"
        class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
      >
        <div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900">
          <div>
            <a href="#!" class="float-right w-4 h-4 i-tabler-x" />
            <h2 class="text-xl">Add Actor Kind</h2>
          </div>
          <form method="post" class="flex flex-col gap-4 justify-center">
            <input type="hidden" name="game_id" value={game.id} />
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
              <button
                type="submit"
                name="submit"
                value="add_actor"
                class={Button()}
              >
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
