import { Handlers, PageProps } from "$fresh/server.ts";
import { Game } from "~api-models";
import { Head } from "$fresh/runtime.ts";
import funRequest from "~lib/funicular-request.ts";

export const handler: Handlers<void | Game[]> = {
  async GET(_, ctx) {
    return ctx.render(
      await (await funRequest().path("game").fetch()).json(),
    );
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
          {data.map((g) => (
            <tr key={g.id} class="px-4 py-2">
              <td>
                <a href={`games/${g.name}`}>
                  {g.name}
                </a>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
