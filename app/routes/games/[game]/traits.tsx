import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Trait } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";
import funRequest from "~lib/funicular-request.ts";

export const handler: Handlers<Trait[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    const data = await funRequest().path("trait").select([
      "*",
      "game!inner()",
    ]).eq("game.name", game).json();
    return ctx.render(data);
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<Trait[]>,
) {
  return (
    <>
      <Head>
        <title>Funicular - Traits</title>
      </Head>
      <div class="mx-auto">
        <Breadcrumb path={pathname}>
          <span>{game}</span>
          <span>Traits</span>
        </Breadcrumb>
        {data.length > 0
          ? (
            <table class="table-auto border-collapse mx-auto">
              <thead>
                <tr class="px-4 py-2">
                  <th>Name</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                {data.map((t) => (
                  <tr key={t.id} class="px-4 py-2">
                    <td>{t.name}</td>
                    <td>{t.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
          : <p>No traits...</p>}
      </div>
    </>
  );
}
