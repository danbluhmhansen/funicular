import type { ActionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";
import { generators } from "openid-client";
import { AuthClient } from "~/lib/auth";
import { commitSession, getSession } from "~/sessions";

export async function action({ request }: ActionArgs) {
  const client = await AuthClient();
  const session = await getSession(request.headers.get("Cookie"));

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
