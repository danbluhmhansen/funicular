import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Actor, ActorNumSkill, Gear, Skill, Trait } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";

interface ActorQuery extends Actor {
  skill: Skill[];
  trait: Trait[];
  gear: Gear[];
}

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

    const select = [
      "*",
      "...actor_kind!inner(game!inner(),skill(name))",
      "trait:actor_trait(...trait(name))",
      "gear:actor_gear(...gear(name))",
    ].join(",");

    const query = [
      `actor_kind.game.name=eq.${game}`,
      `name=ilike.${actor.replace("-", " ")}`,
    ].join("&");

    const url = `http://localhost:3000/actor?select=${select}&${query}`;

    const actorRes = await fetch(url, {
      headers: {
        Accept: "application/vnd.pgrst.object+json",
      },
    });
    const actorQuery: ActorQuery = await actorRes.json();

    const skills = actorQuery.skill;

    const actorSkillsRes = await fetch(
      `http://localhost:3000/actor_num_skill?actor_id=eq.${actorQuery?.id}`,
    );
    const actorSkills: ActorNumSkill[] = await actorSkillsRes.json();

    if (actorQuery && skills && actorSkills) {
      return ctx.render({
        name: actorQuery.name,
        skills: actorSkills.map((skill, i) => {
          return {
            key: skills[i].name,
            value: skill.value ?? 0,
          };
        }),
      });
    } else if (actorQuery) {
      return ctx.render({ name: actorQuery.name });
    } else {
      return ctx.renderNotFound();
    }
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<
    ActorAggregate
  >,
) {
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
        {data.skills && (
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
        )}
      </div>
    </>
  );
}
