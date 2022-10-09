import type { ActionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";

export async function loader() {
  return redirect("/auth/login");
}

export async function action({ request }: ActionArgs) {
  return "";
}
