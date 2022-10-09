import { Title } from "@funicular/shared";
import type { LoaderArgs } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { getSession } from "~/sessions";

export async function loader({ request }: LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));
  const accessToken = session.get("access_token");

  const response = await fetch(`${process.env["SERVER_URL"]}/test`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  return await response.json();
}

export default function Index() {
  const { test } = useLoaderData();

  return (
    <div className="container">
      <Title size={2}>{test}</Title>
    </div>
  );
}
