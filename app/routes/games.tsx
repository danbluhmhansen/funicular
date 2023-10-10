import { defineRoute } from "$fresh/server.ts";

interface Game {
  name: string;
  slug: string;
}

export default defineRoute(async () => {
  const response = await fetch("http://localhost:3000/game?select=name,slug");
  const games: Game[] = await response.json();
  return (
    <>
      <h1 class="text-xl font-bold">Games</h1>
      <div class="overflow-x-auto relative rounded shadow-md">
        <table class="w-full">
          <thead class="text-xs text-gray-700 uppercase dark:text-gray-400 bg-slate-50 dark:bg-slate-700">
            <tr>
              <th class="p-3 text-center">Name</th>
            </tr>
          </thead>
          <tbody>
            {games.map((game) => (
              <tr class="bg-white border-b last:border-0 dark:bg-slate-800 dark:border-slate-700">
                <td class="p-3 text-center">
                  <a href={`/games/${game.slug}`} class="hover:text-violet">{game.name}</a>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
});
