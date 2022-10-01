import type { MetaFunction, LinksFunction } from "@remix-run/node";
import {
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

// const navigation = [
//   { name: "Home", path: "/" },
//   { name: "Characters", path: "/characters" },
// ];

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
