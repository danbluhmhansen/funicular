import { Handlers, PageProps } from "$fresh/server.ts";
import Character from "~models/character.ts";
import Field from "~models/field.ts";
import CharacterNumericField from "~models/character-numeric-field.ts";

interface CharacterAggregate {
  name: string;
  fields?: {
    key: string;
    value: number;
  }[];
}

export const handler: Handlers<CharacterAggregate | null> = {
  async GET(_, ctx) {
    const { character } = ctx.params;

    const character_resp = await fetch(
      `http://localhost:3000/character?id=eq.${character}&select=name`,
      {
        headers: {
          Accept: "application/vnd.pgrst.object+json",
        },
      },
    );

    const fields_resp = await fetch(
      "http://localhost:3000/field?select=name",
    );

    const character_fields_resp = await fetch(
      `http://localhost:3000/character_numeric_field?character_id=eq.${character}&select=value`,
    );

    if (
      character_resp.status === 200 && fields_resp.status === 200 &&
      character_fields_resp.status === 200
    ) {
      const character: Character = await character_resp.json();
      const fields: Field[] = await fields_resp.json();
      const character_fields: CharacterNumericField[] =
        await character_fields_resp.json();

      return ctx.render({
        name: character.name,
        fields: fields.map((field, i) => {
          return {
            key: field.name,
            value: character_fields[i].value,
          };
        }),
      });
    } else if (character_resp.status === 200) {
      return ctx.render({ name: await character_resp.json() });
    } else {
      return ctx.render(null);
    }
  },
};

export default function Page({ data }: PageProps<CharacterAggregate | null>) {
  if (!data) {
    return (
      <div class="mx-auto">
        <h1>Character not found.</h1>
      </div>
    );
  }

  if (!data.fields) {
    return (
      <div class="mx-auto">
        {data.name}
      </div>
    );
  }

  return (
    <div class="mx-auto">
      {data.name}
      <table>
        <thead>
          <tr>
            {data.fields.map((f) => <th key={f.key}>{f.key}</th>)}
          </tr>
        </thead>
        <tbody>
          <tr>
            {data.fields.map((f) => <td key={f.key}>{f.value}</td>)}
          </tr>
        </tbody>
      </table>
    </div>
  );
}
