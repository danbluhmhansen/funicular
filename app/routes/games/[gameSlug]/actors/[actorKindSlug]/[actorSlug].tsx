import { defineRoute, Handlers } from "$fresh/server.ts";
import { SERVER_URL } from "~utils/env.ts";
import { Button } from "~styles/button.ts";

interface Actor {
  name: string;
  slug: string;
  description?: string;
  kindName: string;
  gameName: string;
}

export const handler: Handlers = {
  async POST(req, { render, params: { gameSlug, actorKindSlug, actorSlug } }) {
    const form = await req.formData();

    const submit = form.get("submit")?.toString();
    form.delete("submit");

    switch (submit) {
      case "edit": {
        const res = await fetch(
          `${SERVER_URL}/actor?actor_kind.game.slug=eq.${gameSlug}&actor_kind.slug=eq.${actorKindSlug}&slug=eq.${actorSlug}&select=slug,actor_kind!inner(game!inner())`,
          {
            method: "PATCH",
            headers: {
              Accept: "application/vnd.pgrst.object+json",
              "Content-Type": "application/json",
              Prefer: "return=representation",
            },
            body: JSON.stringify(Object.fromEntries(form)),
          },
        );
        const { slug }: Actor = await res.json();
        if (actorSlug !== slug) {
          return new Response(null, {
            status: 308,
            headers: {
              location: `/games/${gameSlug}/actors/${actorKindSlug}/${slug}`,
            },
          });
        }
        break;
      }
    }

    return await render();
  },
};

export default defineRoute(
  async (_, { params: { gameSlug, actorKindSlug, actorSlug } }) => {
    const resActor = await fetch(
      `${SERVER_URL}/actor?actor_kind.game.slug=eq.${gameSlug}&actor_kind.slug=eq.${actorKindSlug}&slug=eq.${actorSlug}&select=name,slug,description,...actor_kind!inner(kindName:name,...game!inner(gameName:name))`,
      { headers: { Accept: "application/vnd.pgrst.object+json" } },
    );
    const actor: Actor = await resActor.json();

    return (
      <>
        <ol class="flex flex-row">
          <li>
            <a href={`/games/${gameSlug}`} class="hover:text-violet-500">
              {actor.gameName}
            </a>
          </li>
          <li class="flex flex-row justify-center items-center">
            <div class="i-tabler-chevron-right" />
          </li>
          <li>
            <a
              href={`/games/${gameSlug}/actors/${actorKindSlug}`}
              class="hover:text-violet-500"
            >
              {actor.kindName}
            </a>
          </li>
        </ol>

        <div class="flex flex-row gap-2 justify-center items-center">
          <h1 class="text-xl font-bold">{actor.name}</h1>
          <a href="#edit" class={Button("yellow")}>
            <div class="w-4 h-4 i-tabler-pencil" />
          </a>
        </div>

        <dialog
          id="edit"
          class="hidden inset-0 z-10 justify-center items-center w-full h-full target:flex bg-black/50 backdrop-blur-sm"
        >
          <div class="flex z-10 flex-col gap-4 p-4 max-w-sm bg-white rounded border dark:text-white dark:bg-slate-900">
            <div>
              <a href="#!" class="float-right w-4 h-4 i-tabler-x" />
              <h2 class="text-xl">Edit Actor Kind</h2>
            </div>
            <form method="post" class="flex flex-col gap-4 justify-center">
              <input
                type="text"
                name="name"
                placeholder="Name"
                required
                autofocus
                value={actor.name}
                class="bg-transparent rounded invalid:border-red"
              />
              <textarea
                name="description"
                placeholder="Description"
                value={actor.description}
                class="bg-transparent rounded invalid:border-red"
              />
              <div class="flex justify-between">
                <button
                  type="submit"
                  name="submit"
                  value="edit"
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
  },
);
