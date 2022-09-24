import type { ErrorBoundaryComponent } from "@remix-run/node";
import { Link } from "@remix-run/react";
import { useToggle } from "~/hooks/use-toggle";

export const ErrorBoundary: ErrorBoundaryComponent = ({ error }) => {
  return <nav className="navbar">{error.name}</nav>;
};

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
          <span />
          <span />
          <span />
        </a>
      </div>

      <div className={"navbar-menu" + (active ? " is-active" : "")}>
        <div className="navbar-start">
          {navigation.map((n) => (
            <Link key={n.path} to={n.path} className="navbar-item">
              {n.name}
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
