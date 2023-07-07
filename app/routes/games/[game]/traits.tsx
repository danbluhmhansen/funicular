import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Trait } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";

export const handler: Handlers<void | Trait[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await (await fetch(
        `http://localhost:3000/trait?select=*,game!inner()&game.name=eq.${game}`,
      )).json(),
    );
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<void | Trait[]>,
) {
  if (!data) {
    return <h1>No traits...</h1>;
  }

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
        <table class="table-auto border-collapse mx-auto">
          <thead>
            <tr class="px-4 py-2">
              <th>Name</th>
            </tr>
          </thead>
          <tbody>
            {data.map((t) => (
              <tr key={t.id} class="px-4 py-2">
                <td>{t.name}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
}
