import { Handlers, PageProps } from "$fresh/server.ts";
import {
  ActorApi,
  ActorNumSkillApi,
  createConfiguration,
  SkillApi,
} from "../../api-client/index.ts";

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

    const config = createConfiguration();
    const actorApi = new ActorApi(config);
    const actorNumSkillApi = new ActorNumSkillApi(config);
    const skillApi = new SkillApi(config);

    const actors = await actorApi.actorGet({ id: `eq.${id}` });
    const actor = actors ? actors[0] : undefined;

    const skills = await skillApi.skillGet();

    const actorSkills = await actorNumSkillApi.actorNumSkillGet({
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
      <div class="mx-auto">
        <h1>Skills not found.</h1>
      </div>
    );
  }

  if (!data.skills) {
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
            {data.skills.map((s) => <th key={s.key}>{s.key}</th>)}
          </tr>
        </thead>
        <tbody>
          <tr>
            {data.skills.map((s) => <td key={s.key}>{s.value}</td>)}
          </tr>
        </tbody>
      </table>
    </div>
  );
}
