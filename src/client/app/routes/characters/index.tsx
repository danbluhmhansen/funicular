import type { LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type Character from "~/models/character";

export const loader: LoaderFunction = async () => {
  const response = await fetch("https://localhost:7000/graphql", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query:
        "query { characters(top: 10) { id name strength dexterity constitution intelligence wisdom charisma } }",
    }),
  });
  const context = await response.json();
  return context.data.characters;
};

export default function Index() {
  const characters = useLoaderData();

  return (
    <div className="flex flex-col">
      <div className="overflow-x-auto sm:-mx-6 lg:-mx-8">
        <div className="inline-block min-w-full py-2 sm:px-6 lg:px-8">
          <div className="overflow-hidden">
            <table className="min-w-full">
              <thead className="border-b">
                <tr>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Name
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Strength
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Dexterity
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Constitution
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Intelligence
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Wisdom
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Charisma
                  </th>
                </tr>
              </thead>
              <tbody>
                {characters &&
                  characters.map((character: Character) => (
                    <tr key={character.id} className="border-b">
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.name}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.strength}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.dexterity}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.constitution}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.intelligence}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.wisdom}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.charisma}
                      </td>
                    </tr>
                  ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
