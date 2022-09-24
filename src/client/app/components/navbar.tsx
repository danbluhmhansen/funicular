import { Link } from "@remix-run/react";
import { useToggle } from "~/hooks/use-toggle";

export interface NavItem {
  name: string;
  path: string;
}

export default function Navbar({ navigation }: { navigation: NavItem[] }) {
  const [active, toggleActive] = useToggle();

  return (
    <nav className="navbar">
      <div className="navbar-brand">
        <a className="navbar-burger" onClick={toggleActive}>
          <span></span>
          <span></span>
          <span></span>
        </a>
      </div>

      <div className={"navbar-menu" + (active ? " is-active" : "")}>
        <div className="navbar-start">
          {navigation.map((navLink) => (
            <Link key={navLink.path} to={navLink.path} className="navbar-item">
              {navLink.name}
            </Link>
          ))}
        </div>

        <div className="navbar-end">
          <div className="navbar-item">
            <div className="buttons">
              <a className="button is-primary">
                <strong>Sign up</strong>
              </a>
              <a className="button is-light">Log in</a>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
}
