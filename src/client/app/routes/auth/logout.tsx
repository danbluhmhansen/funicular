import type { LoaderArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { destroySession, getSession } from "~/sessions";

export async function loader({ request }: LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));
  return redirect(process.env["SERVER_URL"] + "/connect/logout", {
    headers: {
      "Set-Cookie": await destroySession(session),
    },
  });
}
