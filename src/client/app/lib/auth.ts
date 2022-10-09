import type { BaseClient } from "openid-client";
import { Issuer } from "openid-client";

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
