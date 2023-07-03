import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Trait, traitGet } from "~apis";

export const handler: Handlers<void | Trait[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await traitGet({ select: `*,game!inner()&game.name=eq.${game}'` }),
    );
  },
};

export default function Page({ data }: PageProps<void | Trait[]>) {
  if (!data) {
    return <h1>No traits...</h1>;
  }

  return (
    <>
      <Head>
        <title>Funicular - Traits</title>
      </Head>
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
    </>
  );
}
