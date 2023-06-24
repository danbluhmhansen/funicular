import { Handlers, PageProps } from "$fresh/server.ts";
import Character from "~models/character.ts";

export const handler: Handlers<Character | null> = {
  async GET(_, ctx) {
    const { character } = ctx.params;
    const resp = await fetch(
      `http://localhost:3000/character?id=eq.${character}`,
      {
        headers: {
          Accept: "application/vnd.pgrst.object+json",
        },
      },
    );
    return ctx.render(resp.status === 200 ? await resp.json() : null);
  },
};

export default function Page({ data }: PageProps<Character | null>) {
  if (!data) {
    return (
      <div class="mx-auto">
        <h1>Character not found.</h1>
      </div>
    );
  }

  return <div class="mx-auto">Hello {data.name}.</div>;
}
