import type { GraphQlResponse } from "~/models/graphql/graphql-response";

export async function fetchGraphQl({
  query,
  variables,
}: {
  query: string;
  variables: Record<string, any>;
}): Promise<GraphQlResponse> {
  const response = await fetch(`${process.env.SERVER_URL}/graphql`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query: query,
      variables: variables,
    }),
  });
  return await response.json();
}
