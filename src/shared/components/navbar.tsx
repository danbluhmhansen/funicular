import type { ReactElement } from "react";
import { useToggle } from "../hooks/use-toggle";

export function Navbar({ children }: { children: ReactElement[] }) {
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
        <div className="navbar-start">{children}</div>

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
