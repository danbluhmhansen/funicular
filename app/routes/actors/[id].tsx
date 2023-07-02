import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { actorGet, actorNumSkillGet, skillGet } from "~apis";

interface ActorAggregate {
  name: string;
  skills?: {
    key: string;
    value: number;
  }[];
}

export const handler: Handlers<void | ActorAggregate> = {
  async GET(_, ctx) {
    const { id } = ctx.params;

    const actors = await actorGet({ id: `eq.${id}` });
    const actor = actors ? actors[0] : undefined;

    const skills = await skillGet();

    const actorSkills = await actorNumSkillGet({
      actorId: `eq.${id}`,
    });

    if (actor && skills && actorSkills) {
      return ctx.render({
        name: actor.name,
        skills: actorSkills.map((skill, i) => {
          return {
            key: skills[i].name,
            value: skill.value ?? 0,
          };
        }),
      });
    } else if (actor) {
      return ctx.render({ name: actor.name });
    } else {
      return ctx.render(undefined);
    }
  },
};

export default function Page({ data }: PageProps<void | ActorAggregate>) {
  if (!data) {
    return (
      <>
        <Head>
          <title>Funicular - Not found</title>
        </Head>
        <div class="mx-auto">
          <h1>Actor not found.</h1>
        </div>
      </>
    );
  }

  if (!data.skills) {
    return (
      <>
        <Head>
          <title>Funicular - {data.name}</title>
        </Head>
        <div class="mx-auto">
          {data.name}
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
        <table>
          <thead>
            <tr>
              {data.skills.map((s) => (
                <th key={s.key} class="px-4 py-2">{s.key}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            <tr>
              {data.skills.map((s) => (
                <td key={s.key} class="px-4 py-2">{s.value}</td>
              ))}
            </tr>
          </tbody>
        </table>
      </div>
    </>
  );
}
