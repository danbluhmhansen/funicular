import { Navbar } from "@funicular/shared";
import type { MetaFunction, LinksFunction } from "@remix-run/node";
import {
  Form,
  Link,
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import styles from "../styles/styles.css";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Funicular",
  viewport: "width=device-width,initial-scale=1",
});

export const links: LinksFunction = () => [{ rel: "stylesheet", href: styles }];

const navigation = [
  { name: "Home", path: "/" },
  { name: "Characters", path: "/characters" },
];

function PrimaryButton() {
  return (
    <Link to="/" className="button is-primary">
      Register
    </Link>
  );
}

function SecondaryButton() {
  return (
    <Form method="post" action="/auth/login">
      <button type="submit" className="button is-light">
        Login
      </button>
    </Form>
  );
}

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <Navbar
          primaryButton={<PrimaryButton />}
          secondaryButton={<SecondaryButton />}
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
