import type { MetaFunction, LinksFunction } from "@remix-run/node";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import type { NavLink } from "./components/navbar";
import Navbar from "./components/navbar";
import styles from "../styles/styles.css";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Funicular",
  viewport: "width=device-width,initial-scale=1",
});

export const links: LinksFunction = () => [{ rel: "stylesheet", href: styles }];

const navigation: NavLink[] = [
  { name: "Home", path: "/" },
  { name: "Characters", path: "/characters" },
];

export default function App() {
  return (
    <html lang="en">
      <head>
        <Meta />
        <Links />
      </head>
      <body>
        <Navbar navigation={navigation} />
        <Outlet />
        <ScrollRestoration />
        <Scripts />
        <LiveReload />
      </body>
    </html>
  );
}
