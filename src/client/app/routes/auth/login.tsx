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
    client_secret: "fb485b1e-d4c6-427a-967b-8a0ebedb8c75",
    redirect_uris: ["http://localhost:3000/auth/callback"],
    response_types: ["code"],
  });

  const verifier = generators.codeVerifier();
  const nonce = generators.nonce();
  session.set("code_verifier", verifier);
  session.set("nonce", nonce);

  return redirect(
    client.authorizationUrl({
      scope: "openid email profile",
      code_challenge: generators.codeChallenge(verifier),
      code_challenge_method: "S256",
      nonce,
      response_mode: "form_post",
    }),
    {
      headers: {
        "Set-Cookie": await commitSession(session),
      },
    }
  );
}
