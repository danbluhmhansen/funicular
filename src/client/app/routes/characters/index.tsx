import type { ErrorBoundaryComponent, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import Pagination from "~/components/pagination";
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
      query: `
query {
  characters(count: true, top: 10) {
    id
    name
    strength
    dexterity
    constitution
    intelligence
    wisdom
    charisma
  }
}`,
    }),
  });
  return await response.json();
};

export default function Index() {
  const context = useLoaderData();

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
      <tfoot>
        <tr>
          <td colSpan={7}>
            <Pagination
              count={context.extensions.count}
              pageSizes={[5, 10, 25]}
            />
          </td>
        </tr>
      </tfoot>
      <tbody>
        {context.data.characters &&
          context.data.characters.map((character: Character) => (
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
