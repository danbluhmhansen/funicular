import { Handlers, PageProps } from "$fresh/server.ts";
import { Head } from "$fresh/runtime.ts";
import { Actor, Gear, Skill, Trait } from "~api-models";
import { Breadcrumb } from "~components/breadcrumb.tsx";
import funRequest from "~lib/funicular-request.ts";

interface SkillMap extends Skill {
  value: number;
}

interface ActorMap extends Actor {
  gears: Gear[] | undefined;
  skills: SkillMap[] | undefined;
  traits: Trait[] | undefined;
}

export const handler: Handlers<ActorMap> = {
  async GET(_, ctx) {
    const { game, actor } = ctx.params;

    const data = await funRequest().path("actor").select([
      "name",
      "gears:actor_gear(...gear(name))",
      "skills:actor_num_skill(...skill(name),value)",
      "traits:actor_trait(...trait(name))",
      "...actor_kind!inner(game!inner())",
    ]).eq("actor_kind.game.name", game).ilike("name", actor.replace("-", " "))
      .single().json();

    return data ? ctx.render(data) : ctx.renderNotFound();
  },
};

export default function Page(
  { data, params: { game }, url: { pathname } }: PageProps<
    ActorMap
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
        <h1 class="text-xl">{data.name}</h1>
        {data.skills && (
          <table>
            <thead>
              <tr>
                {data.skills.map((s) => (
                  <th key={s.name} class="px-4 py-2">{s.name}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              <tr>
                {data.skills.map((s) => (
                  <td key={s.name} class="px-4 py-2">{s.value}</td>
                ))}
              </tr>
            </tbody>
          </table>
        )}
        {data.gears && (
          <>
            <h2 class="text-lg">Gear</h2>
            <ul>
              {data.gears.map((g) => <li key={g.name}>{g.name}</li>)}
            </ul>
          </>
        )}
        {data.traits && (
          <>
            <h2 class="text-lg">Traits</h2>
            <ul>
              {data.traits.map((t) => <li key={t.name}>{t.name}</li>)}
            </ul>
          </>
        )}
      </div>
    </>
  );
}
