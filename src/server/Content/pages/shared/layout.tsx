import { Navbar } from "@funicular/shared";
import { createRoot } from "react-dom/client";

const signedIn = globalThis.signedIn;

const navigation = [
  { name: "Home", path: "/" },
  { name: "About", path: "/home/about" },
  { name: "Contact", path: "/home/contact" },
];

function PrimaryButton() {
  return (
    <a
      href={signedIn ? "/manage" : "/account/register"}
      className="button is-primary"
    >
      {signedIn ? "Profile" : "Register"}
    </a>
  );
}

function SecondaryButton() {
  return signedIn ? (
    <form method="post" action="/account/logoff">
      <input
        type="hidden"
        name="__RequestVerificationToken"
        value={globalThis.antiforgeryToken}
      />
      <input type="submit" value="Logout" className="button is-light" />
    </form>
  ) : (
    <a href="/account/login" className="button is-light">
      Login
    </a>
  );
}

export default function Layout() {
  return (
    <Navbar
      primaryButton={<PrimaryButton />}
      secondaryButton={<SecondaryButton />}
    >
      {navigation.map((n) => (
        <a key={n.path} href={n.path} className="navbar-item">
          {n.name}
        </a>
      ))}
    </Navbar>
  );
}

createRoot(document.querySelector("#navbar") as HTMLElement).render(<Layout />);
