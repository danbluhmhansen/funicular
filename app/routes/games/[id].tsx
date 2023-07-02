import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Game, gameGet } from "~apis";

export const handler: Handlers<void | Game> = {
  async GET(_, ctx) {
    const { id } = ctx.params;
    const games = await gameGet({ id: `eq.${id}` });
    return ctx.render(games ? games[0] : undefined);
  },
};

export default function Page({ data }: PageProps<void | Game>) {
  if (!data) {
    return (
      <>
        <Head>
          <title>Funicular - Not found</title>
        </Head>
        <div class="mx-auto">
          <h1>Game not found.</h1>
        </div>
      </>
    );
  }

  return (
    <>
      <Head>
        <title>Funicular - {data.name}</title>
      </Head>
      <div class="mx-auto">
        {data.name}
        <ul>
          <li>
            <a href={`${data.id}/skills`}>Skills</a>
          </li>
        </ul>
      </div>
    </>
  );
}
