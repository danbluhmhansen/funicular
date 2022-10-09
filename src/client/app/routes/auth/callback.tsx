import type { ActionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { Issuer } from "openid-client";
import { commitSession, getSession } from "~/sessions";

export async function loader() {
  return redirect("/auth/login");
}

export async function action({ request }: ActionArgs) {
  const session = await getSession(request.headers.get("Cookie"));

  const issuer = await Issuer.discover(
    process.env["SERVER_URL"] ?? "https://localhost:7000"
  );

  const client = new issuer.Client({
    client_id: "default",
    client_secret: "fb485b1e-d4c6-427a-967b-8a0ebedb8c75",
    redirect_uris: ["http://localhost:3000/auth/callback"],
    response_types: ["code"],
  });

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

  session.set("access_token", tokenSet.access_token);

  return redirect("/", {
    headers: {
      "Set-Cookie": await commitSession(session),
    },
  });
}
