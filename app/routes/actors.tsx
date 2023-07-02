import { Handlers, PageProps } from "$fresh/server.ts";
import { actorGet } from "~apis";
import { Actor } from "~api-models/Actor.ts";
import { Head } from "$fresh/runtime.ts";

export const handler: Handlers<void | Actor[]> = {
  async GET(_, ctx) {
    return ctx.render(await actorGet());
  },
};

export default function Page({ data }: PageProps<void | Actor[]>) {
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
          {data.map((c) => (
            <tr key={c.id} class="px-4 py-2">
              <td>
                <a href={`/actors/${c.id}`}>
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
