import { defineRoute } from "$fresh/server.ts";

export default defineRoute(async () => {
  const response = await fetch("http://localhost:3000/game");
  const games = await response.json();
  console.log(games);
  return (
    <>
      <h1 class="text-4xl font-bold">Games</h1>
      <table>
        <thead>
          <tr>
            <th>Name</th>
          </tr>
        </thead>
        <tbody>
          {games.map((game) => <tr><td>{game.name}</td></tr>)}
        </tbody>
      </table>
    </>
  );
});
