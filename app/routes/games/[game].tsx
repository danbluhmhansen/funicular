import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Game } from "~api-models";
import funRequest from "~lib/funicular-request.ts";

export const handler: Handlers<void | Game> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    const data = await funRequest().path("game").eq("name", game).single()
      .json();
    return data ? ctx.render(data) : ctx.renderNotFound();
  },
};

export default function Page({ data }: PageProps<Game>) {
  return (
    <>
      <Head>
        <title>Funicular - {data.name}</title>
      </Head>
      <div class="mx-auto">
        <h1 class="text-xl">{data.name}</h1>
        <ul>
          <li>
            <a href={`${data.name}/actors`} class="hover:underline">Actors</a>
          </li>
          <li>
            <a href={`${data.name}/skills`} class="hover:underline">Skills</a>
          </li>
          <li>
            <a href={`${data.name}/traits`} class="hover:underline">Traits</a>
          </li>
        </ul>
      </div>
    </>
  );
}
