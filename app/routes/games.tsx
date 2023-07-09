import { Handlers, PageProps } from "$fresh/server.ts";
import { Button } from "~components/button.tsx";
import { Game } from "~api-models";
import { Head } from "$fresh/runtime.ts";
import funRequest from "~lib/funicular-request.ts";
import { TbPlus } from "react-icons/tb";

export const handler: Handlers<Game[]> = {
  async GET(_, ctx) {
    const data = await funRequest().path("game").json();
    return ctx.render(data);
  },
};

export default function Page({ data }: PageProps<Game[]>) {
  return (
    <>
      <Head>
        <title>Funicular - Games</title>
      </Head>
      <div class="mx-auto">
        <div class="flex flex-row items-center justify-between space-x-4 p-4">
          <Button>
            <TbPlus class="mr-1" />
            Add
          </Button>
        </div>
        {data.length > 0
          ? (
            <table class="table-auto border-collapse mx-auto">
              <thead>
                <tr class="px-4 py-2">
                  <th>Name</th>
                </tr>
              </thead>
              <tbody>
                {data.map((g) => (
                  <tr key={g.id} class="px-4 py-2">
                    <td>
                      <a href={`games/${g.name}`} class="hover:underline">
                        {g.name}
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
          : <p>No games...</p>}
      </div>
    </>
  );
}
