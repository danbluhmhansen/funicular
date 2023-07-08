import { Handlers, PageProps } from "$fresh/server.ts";
import { Actor } from "~api-models";
import { Head } from "$fresh/runtime.ts";
import { slug } from "https://deno.land/x/slug@v1.1.0/mod.ts";
import { Breadcrumb } from "~components/breadcrumb.tsx";
import funRequest from "~lib/funicular-request.ts";

export const handler: Handlers<void | Actor[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await (await funRequest().path("actor").select([
        "*",
        "actor_kind!inner(game!inner())",
      ]).eq("actor_kind.game.name", game).fetch()).json(),
    );
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<
    void | Actor[]
  >,
) {
  if (!data) {
    return <h1>No actors...</h1>;
  }

  return (
    <>
      <Head>
        <title>Funicular - Actors</title>
      </Head>
      <div class="mx-auto">
        <Breadcrumb path={pathname}>
          <span>{game}</span>
          <span>Actors</span>
        </Breadcrumb>
        <table class="table-auto border-collapse">
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
                    href={`/games/${game}/actors/${
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
      </div>
    </>
  );
}
