import type { ErrorBoundaryComponent, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import Pagination from "~/components/pagination";
import { fetchGraphQl } from "~/lib/graphql";
import type Character from "~/models/character";
import type { GraphQlResponse } from "~/models/graphql/graphql-response";

export const ErrorBoundary: ErrorBoundaryComponent = ({ error }) => {
  return (
    <article className="message is-danger">
      <div className="message-header">
        <p>{error.name}</p>
      </div>
      <div className="message-body">
        <p>{error.message}</p>
        <p>{error.stack && error.stack}</p>
      </div>
    </article>
  );
};

export const loader: LoaderFunction = async ({ request }) => {
  const url = new URL(request.url);
  const page = +(url.searchParams.get("page") ?? "1");
  const pageSize = +(url.searchParams.get("pageSize") ?? "10");

  return await fetchGraphQl({
    query: `
      query CharactersQuery($skip: Int, $top: Int) {
        characters(count: true, skip: $skip, top: $top) {
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
    variables: {
      skip: (page - 1) * pageSize,
      top: pageSize,
    },
  });
};

const headers = [
  "Name",
  "Strength",
  "Dexterity",
  "Constitution",
  "Intelligence",
  "Wisdom",
  "Charisma",
];

export default function Index() {
  const { data, extensions } = useLoaderData<GraphQlResponse>();
  const characters: Character[] = data.characters;
  const count: number = extensions.count;

  return (
    <div className="container">
      <h3 className="title">Characters</h3>
      <table className="table">
        <thead>
          <tr>
            {headers.map((header) => (
              <th key={header}>{header}</th>
            ))}
          </tr>
        </thead>
        <tfoot>
          <tr>
            <td colSpan={7}>
              <Pagination count={count} pageSizes={[5, 10, 25]} />
            </td>
          </tr>
        </tfoot>
        <tbody>
          {characters &&
            characters.map((character) => (
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
    </div>
  );
}
