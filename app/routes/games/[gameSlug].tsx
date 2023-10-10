import { defineRoute } from "$fresh/server.ts";

interface Game {
  name: string;
  slug: string;
}

export default defineRoute(async (_, { params: { gameSlug } }) => {
  const response = await fetch(
    `http://localhost:3000/game?slug=eq.${gameSlug}&select=name,slug`,
    { headers: { Accept: "application/vnd.pgrst.object+json" } },
  );
  const game: Game = await response.json();
  return (
    <>
      <h1 class="text-xl font-bold">{game.name}</h1>
    </>
  );
});
