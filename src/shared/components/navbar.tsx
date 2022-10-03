import type { ReactElement } from "react";
import { useToggle } from "../hooks/use-toggle";

export function Navbar({
  children,
  primaryButton,
  secondaryButton,
}: {
  children: ReactElement | ReactElement[] | undefined;
  primaryButton: ReactElement | undefined;
  secondaryButton: ReactElement | undefined;
}) {
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
        {primaryButton && secondaryButton && (
          <div className="navbar-end">
            <div className="navbar-item">
              <div className="buttons">
                {primaryButton}
                {secondaryButton}
              </div>
            </div>
          </div>
        )}
      </div>
    </nav>
  );
}
