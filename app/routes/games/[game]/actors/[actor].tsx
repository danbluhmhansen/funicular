import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { actorGet, actorNumSkillGet, skillGet } from "~apis";
import { Breadcrumb } from "~components/breadcrumb.tsx";

interface ActorAggregate {
  name: string;
  skills?: {
    key: string;
    value: number;
  }[];
}

export const handler: Handlers<void | ActorAggregate> = {
  async GET(_, ctx) {
    const { game, actor } = ctx.params;

    const actors = await actorGet({
      select:
        `*,actor_kind!inner(game!inner())&actor_kind.game.name=eq.${game}&name=ilike.${
          actor.replace("-", " ")
        }`,
    });
    const actorModel = actors ? actors[0] : undefined;

    const skills = await skillGet({
      select: `*,game!inner()&game.name=eq.${game}'`,
    });

    const actorSkills = await actorNumSkillGet({
      select: `*,game!inner()&game.name=eq.${game}`,
      actorId: `eq.${actorModel?.id}`,
    });

    if (actorModel && skills && actorSkills) {
      return ctx.render({
        name: actorModel.name,
        skills: actorSkills.map((skill, i) => {
          return {
            key: skills[i].name,
            value: skill.value ?? 0,
          };
        }),
      });
    } else if (actorModel) {
      return ctx.render({ name: actorModel.name });
    } else {
      return ctx.render(undefined);
    }
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<
    void | ActorAggregate
  >,
) {
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
        <Breadcrumb path={pathname}>
          <span>{game}</span>
          <span>Actors</span>
          <span>{data.name}</span>
        </Breadcrumb>
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
