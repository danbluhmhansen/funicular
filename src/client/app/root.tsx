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

const navigation = [{ name: "Characters", path: "/characters" }];

function Nav() {
  return (
    <Navbar
      brand={
        <Link to="/" className="navbar-item">
          Funicular
        </Link>
      }
      primaryButton={
        <Link to="/" className="button is-primary">
          Register
        </Link>
      }
      secondaryButton={
        <Form method="post" action="/auth/login">
          <button type="submit" className="button is-light">
            Login
          </button>
        </Form>
      }
    >
      {navigation.map((n) => (
        <Link key={n.path} to={n.path} className="navbar-item">
          {n.name}
        </Link>
      ))}
    </Navbar>
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
        <Nav />
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
