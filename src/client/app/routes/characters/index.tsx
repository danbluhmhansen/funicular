import type { ErrorBoundaryComponent, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type Character from "~/models/character";

export const ErrorBoundary: ErrorBoundaryComponent = ({ error }) => {
  return (
    <div>
      <h1>{error.name}</h1>
      <p>{error.message}</p>
      {error.stack && <p>{error.stack}</p>}
    </div>
  );
};

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
    <table className="table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Strength</th>
          <th>Dexterity</th>
          <th>Constitution</th>
          <th>Intelligence</th>
          <th>Wisdom</th>
          <th>Charisma</th>
        </tr>
      </thead>
      <tbody>
        {characters &&
          characters.map((character: Character) => (
            <tr key={character.id}>
              <td>{character.name}</td>
              <td>{character.strength}</td>
              <td>{character.dexterity}</td>
              <td>{character.constitution}</td>
              <td>{character.intelligence}</td>
              <td>{character.wisdom}</td>
              <td>{character.charisma}</td>
            </tr>
          ))}
      </tbody>
    </table>
  );
}
