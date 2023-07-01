import { Handlers, PageProps } from "$fresh/server.ts";
import { ActorApi, createConfiguration } from "~apis";
import { Actor } from "~api-models/Actor.ts";
import { Head } from "$fresh/runtime.ts";

export const handler: Handlers<void | Actor[]> = {
  async GET(_, ctx) {
    const api = new ActorApi(createConfiguration());
    return ctx.render(await api.actorGet());
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
          <tr>
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {data.map((c) => (
            <tr key={c.id}>
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
