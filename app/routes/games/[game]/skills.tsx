import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Skill, skillGet } from "~apis";

export const handler: Handlers<void | Skill[]> = {
  async GET(_, ctx) {
    const { game } = ctx.params;
    return ctx.render(
      await skillGet({ select: `*,game!inner()&game.name=eq.${game}'` }),
    );
  },
};

export default function Page({ data }: PageProps<void | Skill[]>) {
  if (!data) {
    return <h1>No skills...</h1>;
  }

  return (
    <>
      <Head>
        <title>Funicular - Skills</title>
      </Head>
      <table class="table-auto border-collapse mx-auto">
        <thead>
          <tr class="px-4 py-2">
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {data.map((s) => (
            <tr key={s.id} class="px-4 py-2">
              <td>{s.name}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}
