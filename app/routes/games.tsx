import { Handlers, PageProps } from "$fresh/server.ts";
import { Game, gameGet } from "~apis";
import { Head } from "$fresh/runtime.ts";

export const handler: Handlers<void | Game[]> = {
  async GET(_, ctx) {
    return ctx.render(await gameGet());
  },
};

export default function Page({ data }: PageProps<void | Game[]>) {
  if (!data) {
    return <h1>No games...</h1>;
  }

  return (
    <>
      <Head>
        <title>Funicular - Games</title>
      </Head>
      <table class="table-auto border-collapse mx-auto">
        <thead>
          <tr class="px-4 py-2">
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {data.map((c) => (
            <tr key={c.id} class="px-4 py-2">
              <td>
                <a href={`games/${c.id}`}>
                  {c.name}
                </a>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
