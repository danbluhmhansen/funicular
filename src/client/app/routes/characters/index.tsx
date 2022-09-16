import type { LoaderFunction } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type Character from "~/models/character";

export const loader: LoaderFunction = async () => {
  return json([
    { id: "d7d2bbdd-f9b1-420d-a7bc-049e3294820e", name: "Foo" },
    { id: "6c2cc8c5-46f1-4118-9531-6284fcb2ab02", name: "Bar" },
    { id: "02fc2dbd-75cf-451a-b929-6e2b6a164f65", name: "Obi" },
    { id: "c38f42b2-3df8-487a-b69d-bba4a8067d05", name: "Wan" },
  ]);
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
                    #
                  </th>
                  <th
                    scope="col"
                    className="px-6 py-4 text-left text-sm font-medium"
                  >
                    Name
                  </th>
                </tr>
              </thead>
              <tbody>
                {characters &&
                  characters.map((character: Character) => (
                    <tr key={character.id} className="border-b">
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-medium">
                        {character.id}
                      </td>
                      <td className="whitespace-nowrap px-6 py-4 text-sm font-light">
                        {character.name}
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
