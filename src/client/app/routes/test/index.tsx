import { Title } from "@funicular/shared";
import type { LoaderArgs } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { authFetch } from "~/lib/auth";

export async function loader({ request }: LoaderArgs) {
  return await authFetch(request, `${process.env["SERVER_URL"]}/test`, "test");
}

export default function Index() {
  const { test } = useLoaderData();

  return (
    <div className="container">
      <Title size={2}>{test}</Title>
    </div>
  );
}
