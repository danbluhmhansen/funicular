import type { MetaFunction, LinksFunction, LoaderArgs } from "@remix-run/node";
import {
  Link,
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  useLoaderData,
} from "@remix-run/react";
import styles from "./styles/styles.css";
import { getSession } from "./sessions";
import { Navbar } from "./components/navbar";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Funicular",
  viewport: "width=device-width,initial-scale=1",
});

export const links: LinksFunction = () => [{ rel: "stylesheet", href: styles }];

export async function loader({ request }: LoaderArgs) {
  const session = await getSession(request.headers.get("Cookie"));
  const accessToken = session.get("access_token");
  return !accessToken ? false : true;
}

const navigation = [
  { name: "Characters", path: "/characters" },
  { name: "Test", path: "/test" },
];

export default function App() {
  const signedIn: boolean = useLoaderData();
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <Navbar
          brand={
            <Link to="/" className="navbar-item">
              Funicular
            </Link>
          }
          primaryButton={
            signedIn ? (
              <Link to="/auth/logout" className="button is-primary">
                Log out
              </Link>
            ) : undefined
          }
        >
          {navigation.map((n) => (
            <Link key={n.path} to={n.path} className="navbar-item">
              {n.name}
            </Link>
          ))}
        </Navbar>
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
