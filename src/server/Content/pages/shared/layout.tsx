import { Navbar } from "@funicular/shared";
import { createRoot } from "react-dom/client";

const navigation = [
  { name: "Home", path: "/" },
  { name: "About", path: "/home/about" },
  { name: "Contact", path: "/home/contact" },
];

export default function Layout() {
  return (
    <Navbar>
      {navigation.map((n) => (
        <a key={n.path} href={n.path} className="navbar-item">
          {n.name}
        </a>
      ))}
    </Navbar>
  );
}

createRoot(document.querySelector("#navbar") as HTMLElement).render(<Layout />);
