import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Game } from "~api-models";

export const handler: Handlers<void | Game> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    const games =
      await (await fetch(`http://localhost:3000/game?name=eq.${game}`)).json();
    return games && games[0] ? ctx.render(games[0]) : ctx.renderNotFound();
  },
};

export default function Page({ data }: PageProps<Game>) {
  return (
    <>
      <Head>
        <title>Funicular - {data.name}</title>
      </Head>
      <div class="mx-auto">
        {data.name}
        <ul>
          <li>
            <a href={`${data.name}/actors`}>Actors</a>
          </li>
          <li>
            <a href={`${data.name}/skills`}>Skills</a>
          </li>
          <li>
            <a href={`${data.name}/traits`}>Traits</a>
          </li>
        </ul>
      </div>
    </>
  );
}
