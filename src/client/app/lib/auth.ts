import { redirect } from "@remix-run/node";
import type { BaseClient } from "openid-client";
import { Issuer } from "openid-client";
import { getSession } from "~/sessions";

let issuer: Issuer<BaseClient>;

export async function DiscoverIssuer() {
  if (issuer) return issuer;
  issuer = await Issuer.discover(
    process.env["SERVER_URL"] ?? "https://localhost:7000"
  );
  return issuer;
}

export async function AuthClient() {
  const iss = await DiscoverIssuer();
  return new iss.Client({
    client_id: "default",
    client_secret: "fb485b1e-d4c6-427a-967b-8a0ebedb8c75",
    redirect_uris: ["http://localhost:3000/auth/callback"],
    response_types: ["code"],
  });
}

export async function authFetch(
  request: Request,
  input: RequestInfo | URL,
  redirectUrl?: string,
  init?: RequestInit
): Promise<any> {
  const session = await getSession(request.headers.get("Cookie"));
  const accessToken = session.get("access_token");
  if (!accessToken) return redirect(`/auth/login/${redirectUrl}`);
  if (!init) init = {};
  init.headers = { ...init.headers, Authorization: `Bearer ${accessToken}` };
  const response = await fetch(input, init);
  return await response.json();
}
