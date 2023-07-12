import { Handlers, PageProps } from "$fresh/server.ts";
import { Game } from "~api-models";
import { Head } from "$fresh/runtime.ts";
import funRequest from "~lib/funicular-request.ts";
import GameGrid from "~islands/game-grid.tsx";
import { useSignal } from "@preact/signals";
import { auditTrack } from "~models/audit.ts";
import { Button } from "~components/button.tsx";

export const handler: Handlers<Game[]> = {
  async GET(_, ctx) {
    return ctx.render(await funRequest().path("game").json());
  },
};

export default function Page({ data }: PageProps<Game[]>) {
  const audit = useSignal(data.map((g) => auditTrack(g)));
  return (
    <>
      <Head>
        <title>Funicular - Games</title>
      </Head>
      <div class="mx-auto">
        <GameGrid audits={audit} />
      </div>
    </>
  );
}
