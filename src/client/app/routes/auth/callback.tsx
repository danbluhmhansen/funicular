import type { ActionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { AuthClient } from "~/lib/auth";
import { commitSession, getSession } from "~/sessions";

export async function loader() {
  return redirect("/auth/login");
}

export async function action({ request }: ActionArgs) {
  const client = await AuthClient();
  const session = await getSession(request.headers.get("Cookie"));

  const params = new URLSearchParams(await request.text());
  const tokenSet = await client.callback(
    "http://localhost:3000/auth/callback",
    {
      code: params.get("code") ?? "",
      iss: params.get("iss"),
    },
    {
      code_verifier: session.get("code_verifier"),
      nonce: session.get("nonce"),
    }
  );

  console.log();
  console.log(tokenSet);
  console.log();

  session.set("access_token", tokenSet.access_token);
  return redirect("/", {
    headers: {
      "Set-Cookie": await commitSession(session),
    },
  });
}
