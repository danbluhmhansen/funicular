import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Skill } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";
import funRequest from "~lib/funicular-request.ts";

export const handler: Handlers<Skill[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    const data = await funRequest().path("skill").select([
      "*",
      "game!inner()",
    ]).eq("game.name", game).json();
    console.log(data);
    return ctx.render(data);
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<Skill[]>,
) {
  return (
    <>
      <Head>
        <title>Funicular - Skills</title>
      </Head>
      <div class="mx-auto">
        <Breadcrumb path={pathname}>
          <span>{game}</span>
          <span>Skills</span>
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
                {data.map((s) => (
                  <tr key={s.id} class="px-4 py-2">
                    <td>{s.name}</td>
                    <td>{s.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
          : <p>No skills...</p>}
      </div>
    </>
  );
}
