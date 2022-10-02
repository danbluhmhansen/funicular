import { Navbar } from "@funicular/shared";
import { createRoot } from "react-dom/client";

const navigation = [
  { name: "Home", path: "/" },
  { name: "About", path: "/home/about" },
  { name: "Contact", path: "/home/contact" },
];

export default function Layout() {
  return (
    <Navbar
      primaryButton={{ path: "/account/register", name: "Register" }}
      secondaryButton={{ path: "/account/login", name: "Login" }}
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
