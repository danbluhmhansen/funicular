import { Handlers, PageProps } from "$fresh/server.ts";
import { Actor, actorGet } from "~apis";
import { Head } from "$fresh/runtime.ts";
import { slug } from "https://deno.land/x/slug@v1.1.0/mod.ts";

export const handler: Handlers<void | Actor[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await actorGet({
        select:
          `*,actor_kind!inner(game!inner())&actor_kind.game.name=eq.${game}'`,
      }),
    );
  },
};

export default function Page({ data, params }: PageProps<void | Actor[]>) {
  if (!data) {
    return <h1>No actors...</h1>;
  }

  return (
    <>
      <Head>
        <title>Funicular - Actors</title>
      </Head>
      <table class="table-auto border-collapse mx-auto">
        <thead>
          <tr class="px-4 py-2">
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {data.map((a) => (
            <tr key={a.id} class="px-4 py-2">
              <td>
                <a
                  href={`/games/${params.game}/actors/${
                    slug(a.name, { lower: true })
                  }`}
                >
                  {a.name}
                </a>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
