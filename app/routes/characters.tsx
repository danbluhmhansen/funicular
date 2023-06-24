import { Handlers, PageProps } from "$fresh/server.ts";
import Character from "~models/character.ts";

export const handler: Handlers<Character[] | null> = {
  async GET(_, ctx) {
    const resp = await fetch("http://localhost:3000/character");
    return ctx.render(resp.status === 200 ? await resp.json() : null);
  },
};

export default function Page({ data }: PageProps<Character[] | null>) {
  if (!data) {
    return <h1>No characters...</h1>;
  }

  return (
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
              <a href={`/characters/${c.id}`}>
                {c.name}
              </a>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
