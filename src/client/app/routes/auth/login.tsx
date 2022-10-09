import type { ActionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { generators, Issuer } from "openid-client";
import { commitSession, getSession } from "~/sessions";

export async function action({ request }: ActionArgs) {
  const session = await getSession(request.headers.get("Cookie"));
  const issuer = await Issuer.discover(
    process.env["SERVER_URL"] ?? "https://localhost:7000"
  );
  const client = new issuer.Client({
    client_id: "default",
    client_secret: "80a390b8-01b9-4ddf-8051-ff70eb5f15a0",
    redirect_uris: ["http://localhost:3000/auth/callback"],
    response_types: ["code"],
  });
  const code_verifier = generators.codeVerifier();
  session.set("code_verifier", code_verifier);
  const code_challenge = generators.codeChallenge(code_verifier);
  const nonce = generators.nonce();
  return redirect(
    client.authorizationUrl({
      scope: "openid email profile",
      code_challenge,
      code_challenge_method: "S256",
      nonce,
    }),
    {
      headers: {
        "Set-Cookie": await commitSession(session),
      },
    }
  );
}
