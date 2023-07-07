import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Skill } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";

export const handler: Handlers<void | Skill[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await (await fetch(
        `http://localhost:3000/skill?select=*,game!inner()&game.name=eq.${game}`,
      )).json(),
    );
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<void | Skill[]>,
) {
  if (!data) {
    return <h1>No skills...</h1>;
  }

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
        <table class="table-auto border-collapse mx-auto">
          <thead>
            <tr class="px-4 py-2">
              <th>Name</th>
            </tr>
          </thead>
          <tbody>
            {data.map((s) => (
              <tr key={s.id} class="px-4 py-2">
                <td>{s.name}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
}
